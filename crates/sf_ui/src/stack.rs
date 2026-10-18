//! Screen stacks per layer and the input routing rule (ui.md "Canonical
//! technique": Unreal Common UI activatable widget stacks).
//!
//! Each layer holds a stack of open screens. Input goes to the top screen of
//! the highest layer that takes input and has a screen open; closing it
//! falls back to the next screen or layer. `sf_ui_bevy` turns the target's
//! input context into the input crate's active context with the highest
//! priority, so its actions shadow everything below.

use crate::ids::{InputContextId, ScreenId};
use crate::records::{Layer, PausePolicy};
use crate::registry::UiRegistry;

/// An open screen, with what the router needs from its record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenScreen {
    pub id: ScreenId,
    pub layer: Layer,
    pub input: InputContextId,
    pub pause: PausePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum StackError {
    #[error("no screen `{0}` in the UI registry")]
    UnknownScreen(String),
}

/// Open screens of every layer.
#[derive(Clone, Debug, Default)]
pub struct ScreenStack {
    layers: [Vec<OpenScreen>; 5],
    /// Bumped on every change, so the Bevy side reacts only to changes.
    version: u64,
}

impl ScreenStack {
    pub fn new() -> Self {
        Self::default()
    }

    /// Opens a screen on top of its layer. A screen already open moves to the
    /// top instead of opening twice.
    pub fn push(&mut self, registry: &UiRegistry, id: &str) -> Result<&OpenScreen, StackError> {
        let def = registry
            .screen(id)
            .ok_or_else(|| StackError::UnknownScreen(id.to_owned()))?;
        let open = OpenScreen {
            id: def.id.clone(),
            layer: def.def.layer,
            input: def
                .def
                .input
                .clone()
                .unwrap_or_else(|| InputContextId::new(def.id.as_str())),
            pause: def.def.pause,
        };
        let stack = &mut self.layers[open.layer.index()];
        stack.retain(|s| s.id.as_str() != id);
        stack.push(open);
        self.version += 1;
        Ok(stack.last().expect("just pushed"))
    }

    /// Replaces the top screen of the new screen's layer (tab switches,
    /// main menu to load screen). Pushes when the layer is empty.
    pub fn replace_top(
        &mut self,
        registry: &UiRegistry,
        id: &str,
    ) -> Result<&OpenScreen, StackError> {
        let def = registry
            .screen(id)
            .ok_or_else(|| StackError::UnknownScreen(id.to_owned()))?;
        self.layers[def.def.layer.index()].pop();
        self.push(registry, id)
    }

    /// Closes the top screen of a layer.
    pub fn pop(&mut self, layer: Layer) -> Option<OpenScreen> {
        let popped = self.layers[layer.index()].pop();
        if popped.is_some() {
            self.version += 1;
        }
        popped
    }

    /// Closes a screen wherever it is in its stack.
    pub fn close(&mut self, id: &str) -> bool {
        for stack in &mut self.layers {
            if let Some(i) = stack.iter().position(|s| s.id.as_str() == id) {
                stack.remove(i);
                self.version += 1;
                return true;
            }
        }
        false
    }

    /// Closes every screen of a layer.
    pub fn clear(&mut self, layer: Layer) {
        if !self.layers[layer.index()].is_empty() {
            self.layers[layer.index()].clear();
            self.version += 1;
        }
    }

    pub fn top(&self, layer: Layer) -> Option<&OpenScreen> {
        self.layers[layer.index()].last()
    }

    /// Open screens of a layer, bottom to top.
    pub fn layer(&self, layer: Layer) -> &[OpenScreen] {
        &self.layers[layer.index()]
    }

    pub fn is_open(&self, id: &str) -> bool {
        self.layers.iter().flatten().any(|s| s.id.as_str() == id)
    }

    /// The screen that receives input actions: the top screen of the highest
    /// input-taking layer with a screen open.
    pub fn input_target(&self) -> Option<&OpenScreen> {
        Layer::ALL
            .iter()
            .rev()
            .filter(|l| l.takes_input())
            .find_map(|l| self.top(*l))
    }

    /// Whether any open screen pauses the game.
    pub fn pauses_game(&self) -> bool {
        self.layers
            .iter()
            .flatten()
            .any(|s| s.pause == PausePolicy::WhileOpen)
    }

    pub fn version(&self) -> u64 {
        self.version
    }
}
