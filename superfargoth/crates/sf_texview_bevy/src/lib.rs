//! The dev texture viewer: Unreal's `vis` for the textures systems register in
//! [`DebugTextures`] (`sf_diag_bevy` feature `texview`). Console:
//!
//! - `tex list`: name, size, format and description of every registered texture.
//! - `tex show <name> [r|g|b|a|rgb] [min max]`: draws it in the bottom-right corner. A name may be
//!   any unique part of one (`tex show trail`). One channel shows as grey, `rgb` as colour
//!   (default: `r` for one-channel formats, else `rgb`); values are remapped from `[min, max]`
//!   (default `0 1`; `max < min` inverts, e.g. reversed-Z depth `1 0`) to black..white, the grey
//!   level on screen being the remapped value. Nearest texels; NaN and infinities in magenta.
//! - `tex hide`. `vis` is an alias of `tex`.
//!
//! How: while a texture is shown, an `Image` target (`Rgba8Unorm`, the source's size up to
//! [`TARGET_MAX`] on the long side) sits in an `ImageNode`; each frame the render world resolves
//! the source (an `Image` asset's GPU texture, or the owner's render-world closure) and one
//! fullscreen-triangle pass (`texview.wesl`) after the cameras (root `RenderGraph`, after
//! `camera_driver`) fills the target with `textureLoad`, so depth, unfilterable float and integer
//! formats need no copy. The UI shows it the next frame (one frame late, as any dev overlay).
//! Hidden: no target, no node, no pass; the render systems return at once.
//!
//! Not viewable: combined depth+stencil formats (register a depth-only view or copy),
//! multisampled textures, array layers and mips other than the view's first.

use bevy_app::{App, Plugin, Update};
use bevy_asset::{Assets, Handle};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageSampler};
use bevy_render::render_resource::{TextureAspect, TextureFormat, TextureSampleType};
use bevy_ui::widget::ImageNode;
use bevy_ui::{GlobalZIndex, Node, PositionType, Val};
use sf_console_bevy::ConsoleAppExt;
use sf_console_bevy::sf_console::{ArgKind, CommandSpec, Log, Param};
use sf_diag_bevy::texview::{DebugTexture, DebugTextures};

mod render;

#[cfg(test)]
mod shader_tests;
#[cfg(test)]
mod tests;

pub use render::TexViewFrame;

/// The target's long side at most (texels): a 1024² map shows one texel per target texel.
pub const TARGET_MAX: u32 = 1024;
/// The view's height on screen (percent of the window height).
pub const VIEW_HEIGHT_VH: f32 = 45.0;

/// The `tex` console command, the corner view and its render pass. Add it with the console.
pub struct TexViewPlugin;

impl Plugin for TexViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TexView>()
            .init_resource::<DebugTextures>()
            .init_resource::<TexViewFrame>()
            .init_resource::<Display>()
            .add_systems(Update, apply_view.run_if(resource_exists::<Assets<Image>>).run_if(resource_changed::<TexView>));
        register_command(app);
        render::build(app);
    }
}

/// Which channel the view shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    R,
    G,
    B,
    A,
    Rgb,
}

impl Channel {
    pub fn parse(word: &str) -> Option<Self> {
        match word.to_ascii_lowercase().as_str() {
            "r" => Some(Self::R),
            "g" => Some(Self::G),
            "b" => Some(Self::B),
            "a" => Some(Self::A),
            "rgb" => Some(Self::Rgb),
            _ => None,
        }
    }

    /// `texview.wesl` `Params::mode.x`.
    pub fn code(self) -> u32 {
        match self {
            Self::R => 0,
            Self::G => 1,
            Self::B => 2,
            Self::A => 3,
            Self::Rgb => 4,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::R => "r",
            Self::G => "g",
            Self::B => "b",
            Self::A => "a",
            Self::Rgb => "rgb",
        }
    }

    /// `r` for one-channel formats (depth, `R16Float`), else `rgb`.
    pub fn default_for(format: TextureFormat) -> Self {
        if format.components() == 1 { Self::R } else { Self::Rgb }
    }
}

/// How the shader reads the source (one pipeline each).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SampleKind {
    /// Float, unfilterable float and depth: `texture_2d<f32>` (depth binds as unfilterable float).
    Float,
    Uint,
    Sint,
}

impl SampleKind {
    pub fn of(format: TextureFormat) -> Result<Self, String> {
        if format.is_combined_depth_stencil_format() {
            return Err(format!("{format:?} is depth+stencil: register a depth-only view or copy"));
        }
        match format.sample_type(None::<TextureAspect>, None) {
            Some(TextureSampleType::Float { .. } | TextureSampleType::Depth) => Ok(Self::Float),
            Some(TextureSampleType::Uint) => Ok(Self::Uint),
            Some(TextureSampleType::Sint) => Ok(Self::Sint),
            None => Err(format!("{format:?} can't be sampled")),
        }
    }

    /// The shader def (`texview.wesl`), if any.
    pub fn shader_def(self) -> Option<&'static str> {
        match self {
            Self::Float => None,
            Self::Uint => Some("SAMPLE_UINT"),
            Self::Sint => Some("SAMPLE_SINT"),
        }
    }
}

/// A parsed `tex` line.
#[derive(Clone, Debug, PartialEq)]
pub enum TexCommand {
    List,
    Show(ShowArgs),
    Hide,
}

/// `tex show <name> [channel] [min max]` as typed.
#[derive(Clone, Debug, PartialEq)]
pub struct ShowArgs {
    pub name: String,
    pub channel: Option<Channel>,
    pub range: Option<(f32, f32)>,
}

const SHOW_USAGE: &str = "tex show <name> [r|g|b|a|rgb] [min max]";

/// Parses the console's `action` and the rest of the line. No action is `list`.
pub fn parse(action: Option<&str>, rest: Option<&str>) -> Result<TexCommand, String> {
    let words: Vec<&str> = rest.unwrap_or("").split_whitespace().collect();
    let action = action.unwrap_or("list");
    match action {
        "list" | "hide" if !words.is_empty() => Err(format!("`tex {action}` takes no arguments")),
        "list" => Ok(TexCommand::List),
        "hide" => Ok(TexCommand::Hide),
        "show" => parse_show(&words).map(TexCommand::Show),
        other => Err(format!("unknown action {other:?}: list, show or hide")),
    }
}

fn parse_show(words: &[&str]) -> Result<ShowArgs, String> {
    let Some((name, mut tail)) = words.split_first() else {
        return Err(format!("missing name: {SHOW_USAGE}"));
    };
    let mut channel = None;
    if let Some((first, rest)) = tail.split_first()
        && let Some(c) = Channel::parse(first)
    {
        channel = Some(c);
        tail = rest;
    }
    let range = match tail {
        [] => None,
        [lo, hi] => {
            let (lo, hi) = (number(lo)?, number(hi)?);
            if lo.total_cmp(&hi).is_eq() {
                return Err(format!("empty range {lo}..{hi}: min and max must differ"));
            }
            Some((lo, hi))
        }
        _ => return Err(format!("expected [r|g|b|a|rgb] [min max] after the name, got {:?}: {SHOW_USAGE}", tail.join(" "))),
    };
    Ok(ShowArgs { name: (*name).to_owned(), channel, range })
}

fn number(word: &str) -> Result<f32, String> {
    word.parse::<f32>()
        .ok()
        .filter(|v| v.is_finite())
        .ok_or_else(|| format!("bad number {word:?} (channel: r, g, b, a or rgb)"))
}

/// What the view shows (set by `tex show`, cleared by `tex hide`).
#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct TexView {
    pub shown: Option<Shown>,
}

/// One shown texture, resolved against the registry.
#[derive(Clone, Debug, PartialEq)]
pub struct Shown {
    /// The registered name (exact).
    pub name: String,
    pub channel: Channel,
    pub min: f32,
    pub max: f32,
}

/// Resolves `args` against `textures`: the entry, its sample kind, the channel and range defaults.
pub fn resolve(textures: &DebugTextures, args: &ShowArgs) -> Result<Shown, String> {
    let tex = textures.find(&args.name)?;
    SampleKind::of(tex.format)?;
    let (min, max) = args.range.unwrap_or((0.0, 1.0));
    Ok(Shown {
        name: tex.name.clone(),
        channel: args.channel.unwrap_or_else(|| Channel::default_for(tex.format)),
        min,
        max,
    })
}

/// The shader's uniform (`texview.wesl` `Params`): `range` (min, 1 / (max - min), 0, 0) as f32,
/// `mode` (channel, 0, 0, 0) as u32, little-endian.
pub fn params(channel: Channel, min: f32, max: f32) -> [u8; 32] {
    let floats = [min, 1.0 / (max - min), 0.0, 0.0];
    let ints = [channel.code(), 0, 0, 0];
    let mut out = [0u8; 32];
    for (i, word) in floats.iter().map(|v| v.to_le_bytes()).chain(ints.iter().map(|v| v.to_le_bytes())).enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&word);
    }
    out
}

/// The target's size for a source of `size`: the same, or the long side scaled to [`TARGET_MAX`].
pub fn target_size(size: (u32, u32)) -> (u32, u32) {
    let (w, h) = (size.0.max(1), size.1.max(1));
    let long = w.max(h);
    if long <= TARGET_MAX {
        return (w, h);
    }
    let scale = |s: u32| {
        let scaled = (u64::from(s) * u64::from(TARGET_MAX) + u64::from(long) / 2) / u64::from(long);
        u32::try_from(scaled).unwrap_or(TARGET_MAX).max(1)
    };
    (scale(w), scale(h))
}

/// The `tex list` line of `tex`, the log line of a `tex show`.
fn shown_line(shown: &Shown, tex: &DebugTexture) -> String {
    format!(
        "tex: {} {}x{} {:?}, {}, {}..{} (bottom right; `tex hide`)",
        tex.name,
        tex.size.0,
        tex.size.1,
        tex.format,
        shown.channel.label(),
        shown.min,
        shown.max
    )
}

fn register_command(app: &mut App) {
    app.add_console_command(
        CommandSpec::new("tex")
            .alias("vis")
            .help(
                "Show a registered render target in the bottom-right corner: `tex list`, \
                 `tex show <name> [r|g|b|a|rgb] [min max]` (values remapped to black..white), `tex hide`",
            )
            .param(
                Param::new("action", ArgKind::Choice(vec!["list".into(), "show".into(), "hide".into()])).optional(),
            )
            .param(Param::rest("args").optional()),
        |inv| run(inv.ctx, inv.args.str("action"), inv.args.str("args"), inv.log),
    );
}

/// The `tex` command's body.
fn run(world: &mut World, action: Option<&str>, rest: Option<&str>, log: &mut Log) -> Result<(), String> {
    match parse(action, rest)? {
        TexCommand::List => match world.get_resource::<DebugTextures>().filter(|t| !t.is_empty()) {
            Some(textures) => textures.iter().for_each(|t| log.info(&t.summary())),
            None => log.info("tex: no debug textures registered"),
        },
        TexCommand::Hide => {
            world.resource_mut::<TexView>().shown = None;
            log.info("tex: hidden");
        }
        TexCommand::Show(args) => {
            let textures = world.get_resource::<DebugTextures>().ok_or("tex: no debug textures registered")?;
            let shown = resolve(textures, &args)?;
            let tex = textures.find(&shown.name)?;
            log.info(&shown_line(&shown, tex));
            world.resource_mut::<TexView>().shown = Some(shown);
        }
    }
    Ok(())
}

/// The corner view while a texture is shown.
#[derive(Resource, Default)]
struct Display {
    node: Option<Entity>,
    image: Option<Handle<Image>>,
}

/// On a `tex` change: drops the old view, then (if shown) makes the target, the node and the
/// render world's frame.
#[allow(clippy::needless_pass_by_value, clippy::cast_precision_loss)]
fn apply_view(
    view: Res<TexView>,
    textures: Res<DebugTextures>,
    mut display: ResMut<Display>,
    mut frame: ResMut<TexViewFrame>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    if let Some(node) = display.node.take() {
        commands.entity(node).despawn();
    }
    if let Some(image) = display.image.take() {
        images.remove(&image);
    }
    let next = view.shown.as_ref().and_then(|shown| {
        let tex = textures.find(&shown.name).ok()?;
        let kind = SampleKind::of(tex.format).ok()?;
        Some((shown, tex, kind))
    });
    let Some((shown, tex, kind)) = next else {
        if frame.0.is_some() {
            frame.0 = None;
        }
        return;
    };
    let (w, h) = target_size(tex.size);
    let mut image = Image::new_target_texture(w, h, TextureFormat::Rgba8Unorm, None);
    image.sampler = ImageSampler::nearest();
    let target = images.add(image);
    let node = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(8.0),
                bottom: Val::Px(8.0),
                height: Val::Vh(VIEW_HEIGHT_VH),
                aspect_ratio: Some(w as f32 / h as f32),
                ..Node::default()
            },
            ImageNode::new(target.clone()),
            GlobalZIndex(i32::MAX - 2),
        ))
        .id();
    frame.0 = Some(render::FrameSource {
        source: tex.source.clone(),
        kind,
        params: params(shown.channel, shown.min, shown.max),
        target: target.clone(),
    });
    *display = Display { node: Some(node), image: Some(target) };
}
