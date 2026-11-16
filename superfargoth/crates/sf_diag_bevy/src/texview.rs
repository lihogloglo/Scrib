//! Named debug textures (feature `texview`): the registry a texture viewer lists and draws from,
//! as Unreal's `vis` lists its render-target pool. A system that owns a render target registers
//! it once, with a name, a line of description, its size and format, and where to find it:
//!
//! - [`TexSource::Image`]: an `Image` asset (a depth map a camera or shadow view renders into);
//!   the viewer reads its GPU texture from `RenderAssets<GpuImage>`.
//! - [`TexSource::Render`]: a render-world texture with no asset (a compute pass's ping-pong
//!   pair): a closure over the render world that returns this frame's view. The viewer calls it
//!   only while that name is shown, so a registration costs nothing per frame.
//!
//! The viewer (`sf_texview_bevy`, console `tex`) is a separate dev plugin: without it the
//! registry is a list of names and nothing runs.
//!
//! ```ignore
//! app.register_debug_texture(DebugTexture::render(
//!     "surface.trail", "trail ring: R depression (m), G lip (m)", (1024, 1024),
//!     TextureFormat::Rgba16Float, |world| world.get_resource::<TrailGpu>().map(|g| g.current_view().clone()),
//! ));
//! ```

use std::fmt::Write as _;
use std::sync::Arc;

use bevy_app::App;
use bevy_asset::Handle;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_render::render_resource::{TextureFormat, TextureView};

/// A render-world lookup: this frame's view of the texture, `None` while it doesn't exist yet.
pub type TexResolver = Arc<dyn Fn(&World) -> Option<TextureView> + Send + Sync>;

/// Where the viewer finds a registered texture.
#[derive(Clone)]
pub enum TexSource {
    /// An `Image` asset (its GPU texture in `RenderAssets<GpuImage>`).
    Image(Handle<Image>),
    /// A render-world texture; the closure gets the render world.
    Render(TexResolver),
}

/// One registered texture.
#[derive(Clone)]
pub struct DebugTexture {
    /// Lowercase, dotted by system (`surface.trail`); no spaces.
    pub name: String,
    /// One line: what it holds, per channel when that matters, and the units.
    pub description: String,
    /// Width, height in texels.
    pub size: (u32, u32),
    pub format: TextureFormat,
    pub source: TexSource,
}

impl DebugTexture {
    /// An `Image` asset; size and format from its descriptor (read it before the render world
    /// takes a `RENDER_WORLD`-only image's data, e.g. right after `Assets::add`).
    pub fn image(name: &str, description: &str, handle: Handle<Image>, image: &Image) -> Self {
        let desc = &image.texture_descriptor;
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            size: (desc.size.width, desc.size.height),
            format: desc.format,
            source: TexSource::Image(handle),
        }
    }

    /// A render-world texture: `resolve` gets the render world and returns this frame's view.
    pub fn render(
        name: &str,
        description: &str,
        size: (u32, u32),
        format: TextureFormat,
        resolve: impl Fn(&World) -> Option<TextureView> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            size,
            format,
            source: TexSource::Render(Arc::new(resolve)),
        }
    }

    /// `name  1024x1024  Rgba16Float  description`, the `tex list` line.
    pub fn summary(&self) -> String {
        let mut s = String::new();
        let _ = write!(s, "{}  {}x{}  {:?}  {}", self.name, self.size.0, self.size.1, self.format, self.description);
        s
    }
}

/// The registered debug textures, in registration order.
#[derive(Resource, Default, Clone)]
pub struct DebugTextures {
    entries: Vec<DebugTexture>,
}

impl DebugTextures {
    /// Adds `texture`, or replaces the entry of the same name (case-insensitive): a system that
    /// recreates its target registers again. Panics on an empty name or one with whitespace (a
    /// bug in the registering plugin, like a bad console name).
    pub fn register(&mut self, texture: DebugTexture) {
        assert!(
            !texture.name.is_empty() && !texture.name.contains(char::is_whitespace),
            "debug texture name {:?}: not empty, no spaces",
            texture.name
        );
        match self.entries.iter_mut().find(|e| e.name.eq_ignore_ascii_case(&texture.name)) {
            Some(slot) => *slot = texture,
            None => self.entries.push(texture),
        }
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &DebugTexture> {
        self.entries.iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The entry `query` names: its exact name (any case), else the only name containing it
    /// (`trail` for `surface.trail`). Errors name the candidates.
    pub fn find(&self, query: &str) -> Result<&DebugTexture, String> {
        let q = query.to_ascii_lowercase();
        if let Some(e) = self.entries.iter().find(|e| e.name.eq_ignore_ascii_case(&q)) {
            return Ok(e);
        }
        let hits: Vec<&DebugTexture> = self.entries.iter().filter(|e| e.name.to_ascii_lowercase().contains(&q)).collect();
        match hits.as_slice() {
            [one] => Ok(one),
            [] => Err(format!("no debug texture {query:?} (`tex list` lists them)")),
            many => Err(format!(
                "{query:?} is ambiguous: {}",
                many.iter().map(|e| e.name.as_str()).collect::<Vec<_>>().join(", ")
            )),
        }
    }
}

/// Registration from plugins (creates [`DebugTextures`] if needed).
pub trait DebugTextureAppExt {
    fn register_debug_texture(&mut self, texture: DebugTexture) -> &mut Self;
}

impl DebugTextureAppExt for App {
    fn register_debug_texture(&mut self, texture: DebugTexture) -> &mut Self {
        self.world_mut().get_resource_or_init::<DebugTextures>().register(texture);
        self
    }
}
