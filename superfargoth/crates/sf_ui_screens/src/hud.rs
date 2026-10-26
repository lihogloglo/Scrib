//! The HUD (ui.md "Screens": HUD shell with slots): value bars, a compass slot, the crosshair
//! and the "press to use" prompt the action layer reports (`sf_action::Prompt`).
//!
//! The game writes [`HudView`]; the HUD screen root ([`HudScreen`]) mirrors it, bars keyed by
//! id. The prompt's glyph comes from the player's keymap for the last-used device and pad
//! family, so a rebind or a pad pick-up relabels it the same frame.

use bevy_ecs::prelude::*;
use bevy_text::TextColor;
use bevy_ui::prelude::*;
use sf_action::{Prompt, TagId, TargetId};
use sf_ui::ActionRef;
use sf_ui_bevy::{GamepadFamily, InputMode, UiKeymap};
use sf_ui_css::UiClass;

use crate::keyed::KeyedChildren;
use crate::widgets::{ScreenColors, StatBar, classes, label, part, show, spawn_bar};

/// A value bar of the HUD.
#[derive(Clone, Debug, PartialEq)]
pub struct HudBar {
    /// Stable id, also a class on the bar (`health` → `.sf-hud-bar.health`).
    pub id: String,
    /// Shown beside the bar (already localized); empty hides it.
    pub label: String,
    pub current: f32,
    pub max: f32,
}

/// What a "press to use" prompt says.
#[derive(Clone, Debug, PartialEq)]
pub struct UsePrompt {
    /// The verb (`Open`, `Talk`, `Take`), localized.
    pub verb: String,
    /// What it acts on, when it has a name.
    pub target: Option<String>,
    /// A press now is accepted (else the prompt shows greyed: the current move blocks it).
    pub ready: bool,
    /// The keymap action whose glyph the prompt shows (the game's activate action); `None` is a
    /// name-only prompt with no glyph (a target that can be looked at, not used).
    pub action: Option<ActionRef>,
}

/// Names for prompts (the content pack's: what a tag is called, what a target is).
pub trait PromptLabels {
    fn verb(&self, tag: TagId) -> String;
    fn target_name(&self, target: TargetId) -> Option<String>;
}

impl UsePrompt {
    /// The prompt of the local player's `ActionOutput::prompt`.
    pub fn from_action(prompt: &Prompt, labels: &dyn PromptLabels, action: ActionRef) -> Self {
        Self {
            verb: labels.verb(prompt.tag),
            target: labels.target_name(prompt.target),
            ready: prompt.ready,
            action: Some(action),
        }
    }
}

/// The HUD's view-model, written by the game.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct HudView {
    pub bars: Vec<HudBar>,
    /// Facing in degrees clockwise from north; `None` hides the heading (the compass slot stays
    /// for content that fills it).
    pub heading: Option<f32>,
    pub crosshair: bool,
    pub prompt: Option<UsePrompt>,
}

/// On the HUD screen's root (a `UiScreenRoot` on the HUD layer): builds and syncs the HUD.
#[derive(Component, Clone, Copy, Debug, Default)]
#[require(Node)]
pub struct HudScreen;

/// A named anchor content may fill (ui.md "Extension slots": `hud.compass`).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct HudSlot(pub &'static str);

/// The HUD's parts.
#[derive(Component, Debug)]
pub struct HudParts {
    bars: KeyedChildren<String>,
    compass_heading: Entity,
    crosshair: Entity,
    prompt: Entity,
    prompt_glyph: Entity,
    prompt_text: Entity,
    shown: Option<(HudView, Option<String>)>,
}

impl HudParts {
    /// The bar entity of a bar id.
    pub fn bar(&self, id: &str) -> Option<Entity> {
        self.bars.get(&id.to_owned())
    }

    pub fn prompt(&self) -> Entity {
        self.prompt
    }

    pub fn prompt_glyph(&self) -> Entity {
        self.prompt_glyph
    }

    pub fn crosshair(&self) -> Entity {
        self.crosshair
    }
}

pub(crate) fn spawn_hud(
    mut commands: Commands,
    colors: Res<ScreenColors>,
    roots: Query<Entity, Added<HudScreen>>,
) {
    for root in &roots {
        commands.entity(root).insert((
            part("sf-hud"),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Node::default()
            },
        ));
        let bars = commands
            .spawn((
                part("sf-hud-bars"),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let compass = commands
            .spawn((
                part("sf-hud-compass"),
                HudSlot("hud.compass"),
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let compass_heading = commands
            .spawn((
                label("sf-hud-compass-heading", "", &colors),
                ChildOf(compass),
            ))
            .id();
        let crosshair = commands
            .spawn((
                part("sf-hud-crosshair"),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    width: Val::Px(4.0),
                    height: Val::Px(4.0),
                    margin: UiRect::all(Val::Px(-2.0)),
                    ..Node::default()
                },
                BackgroundColor(colors.text),
                ChildOf(root),
            ))
            .id();
        let prompt = commands
            .spawn((
                part("sf-hud-prompt"),
                // A full-width row centred under the crosshair, whatever the text's width.
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Percent(56.0),
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(8.0),
                    display: Display::None,
                    ..Node::default()
                },
                ChildOf(root),
            ))
            .id();
        let prompt_glyph = commands
            .spawn((label("sf-glyph", "", &colors), ChildOf(prompt)))
            .id();
        let prompt_text = commands
            .spawn((label("sf-hud-prompt-text", "", &colors), ChildOf(prompt)))
            .id();
        commands.entity(root).insert(HudParts {
            bars: KeyedChildren::new(bars),
            compass_heading,
            crosshair,
            prompt,
            prompt_glyph,
            prompt_text,
            shown: None,
        });
    }
}

/// An 8-point compass label and whole degrees (`NE 045`).
pub fn heading_label(degrees: f32) -> String {
    const POINTS: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let d = degrees.rem_euclid(360.0);
    // In 0..360, so the casts can't truncate or lose sign.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let point = ((d + 22.5) / 45.0) as usize % 8;
    format!("{} {:03.0}", POINTS[point], d.floor())
}

/// [`HudView`] (and the prompt glyph's device, pad family and keymap) → the HUD's nodes, only
/// when something changed.
#[allow(clippy::too_many_arguments)]
pub(crate) fn sync_hud(
    mut commands: Commands,
    view: Option<Res<HudView>>,
    keymap: Option<Res<UiKeymap>>,
    mode: Option<Res<InputMode>>,
    family: Option<Res<GamepadFamily>>,
    colors: Res<ScreenColors>,
    mut huds: Query<&mut HudParts>,
    mut nodes: Query<&mut Node>,
) {
    let Some(view) = view else {
        return;
    };
    let glyph = view.prompt.as_ref().and_then(|p| {
        let keymap = keymap.as_ref()?;
        let device = mode.as_ref().map(|m| m.0).unwrap_or_default();
        let family = family.as_ref().map(|f| f.0).unwrap_or_default();
        keymap.0.glyph(p.action.as_ref()?, device, family)
    });
    let glyph_label = glyph.as_ref().map(|g| g.label.clone());
    for mut hud in &mut huds {
        let hud = &mut *hud;
        if hud
            .shown
            .as_ref()
            .is_some_and(|(v, g)| v == &*view && g == &glyph_label)
        {
            continue;
        }
        // Bars, keyed by id: a bar that stays keeps its entity; values update in place.
        let colors = &*colors;
        hud.bars.sync(
            &mut commands,
            view.bars.iter().map(|b| b.id.clone()),
            |commands, id| {
                let row = commands
                    .spawn((
                        Name::new("sf-hud-bar-row"),
                        sf_ui_bevy::UiPart("sf-hud-bar-row"),
                        UiClass(format!("sf-hud-bar-row {id}")),
                        Node {
                            column_gap: Val::Px(6.0),
                            align_items: AlignItems::Center,
                            ..Node::default()
                        },
                    ))
                    .id();
                commands.spawn((label("sf-hud-bar-label", "", colors), ChildOf(row)));
                let bar = StatBar {
                    current: 0.0,
                    max: 0.0,
                    kind: id.clone(),
                };
                spawn_bar(commands, row, "sf-hud-bar", bar, colors);
                row
            },
        );
        for bar in &view.bars {
            let Some(row) = hud.bars.get(&bar.id) else {
                continue;
            };
            let (text, current, max) = (bar.label.clone(), bar.current, bar.max);
            commands
                .queue(move |world: &mut World| update_bar_row(world, row, &text, current, max));
        }

        // Compass heading.
        commands.entity(hud.compass_heading).insert(Text::new(
            view.heading.map(heading_label).unwrap_or_default(),
        ));
        if let Ok(mut node) = nodes.get_mut(hud.crosshair) {
            show(&mut node, view.crosshair);
        }
        // Prompt: glyph, verb and target; greyed when not ready.
        if let Ok(mut node) = nodes.get_mut(hud.prompt) {
            show(&mut node, view.prompt.is_some());
        }
        if let Ok(mut node) = nodes.get_mut(hud.prompt_glyph) {
            show(&mut node, glyph.is_some());
        }
        if let Some(prompt) = &view.prompt {
            let text = match &prompt.target {
                Some(target) => format!("{} {target}", prompt.verb),
                None => prompt.verb.clone(),
            };
            let color = if prompt.ready {
                colors.text
            } else {
                colors.muted
            };
            commands
                .entity(hud.prompt_text)
                .insert((Text::new(text), TextColor(color)));
            commands
                .entity(hud.prompt)
                .insert(classes("sf-hud-prompt", &[("blocked", !prompt.ready)]));
            let icon = glyph
                .as_ref()
                .and_then(|g| g.icons.last())
                .map(|i| format!("icon-{i}"));
            commands.entity(hud.prompt_glyph).insert((
                Text::new(glyph_label.clone().unwrap_or_default()),
                classes(
                    "sf-glyph",
                    &[(icon.as_deref().unwrap_or(""), icon.is_some())],
                ),
            ));
        }
        hud.shown = Some((view.clone(), glyph_label.clone()));
    }
}

/// Writes one bar row's label and values (children: label, bar).
fn update_bar_row(world: &mut World, row: Entity, label: &str, current: f32, max: f32) {
    let Some(children) = world.get::<Children>(row).map(|c| c.to_vec()) else {
        return;
    };
    for child in children {
        if let Some(mut text) = world.get_mut::<Text>(child) {
            if text.0 != label {
                label.clone_into(&mut text.0);
            }
        } else if let Some(mut bar) = world.get_mut::<StatBar>(child) {
            let kind = bar.kind.clone();
            bar.set_if_neq(StatBar { current, max, kind });
        }
    }
}
