//! Input names and button glyphs, Bevy-free (ui.md "Input", "Keybinds").
//!
//! Bindings are stored and compared as **input names**, the text form the
//! profile already uses (`[bindings.<context>]`, U1c): `key:Enter`,
//! `key:Ctrl+Shift+KeyS`, `mouse:Left`, `pad:South`, `axis:LeftStickX`,
//! `stick:Left` (both axes of a stick, for 2D actions), `mouse_motion`,
//! `mouse_wheel`, `any_key`. Input names after the `kind:` prefix are the
//! engine's variant names (`KeyCode`, `GamepadButton`, ...); this module only
//! knows the grammar, the device class, modifiers and which names overlap.
//! The engine adapter (`sf_ui_bevy`) maps names to real inputs and validates
//! them through [`crate::Keymap::set_validator`].

use std::fmt;

/// Input device class: keyboard and mouse, or gamepad. Each action has
/// separate binding slots per device (ui.md "Input": per-device bindings).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Device {
    #[default]
    Kbm,
    Gamepad,
}

impl Device {
    pub const ALL: [Device; 2] = [Device::Kbm, Device::Gamepad];

    /// Stable name: profile keys, the stylesheet `input` attribute.
    pub fn name(self) -> &'static str {
        match self {
            Device::Kbm => "kbm",
            Device::Gamepad => "gamepad",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|d| d.name() == name)
    }

    pub(crate) fn index(self) -> usize {
        match self {
            Device::Kbm => 0,
            Device::Gamepad => 1,
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The kind prefix of an input name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputKind {
    Key,
    Mouse,
    Pad,
    Axis,
    Stick,
    MouseMotion,
    MouseWheel,
    AnyKey,
}

impl InputKind {
    pub fn device(self) -> Device {
        match self {
            InputKind::Key
            | InputKind::Mouse
            | InputKind::MouseMotion
            | InputKind::MouseWheel
            | InputKind::AnyKey => Device::Kbm,
            InputKind::Pad | InputKind::Axis | InputKind::Stick => Device::Gamepad,
        }
    }

    fn prefix(self) -> &'static str {
        match self {
            InputKind::Key => "key",
            InputKind::Mouse => "mouse",
            InputKind::Pad => "pad",
            InputKind::Axis => "axis",
            InputKind::Stick => "stick",
            InputKind::MouseMotion => "mouse_motion",
            InputKind::MouseWheel => "mouse_wheel",
            InputKind::AnyKey => "any_key",
        }
    }
}

/// Keyboard modifiers of a key or mouse-button chord, as a bit set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Mods(u8);

impl Mods {
    pub const CTRL: Mods = Mods(1);
    pub const SHIFT: Mods = Mods(2);
    pub const ALT: Mods = Mods(4);
    pub const SUPER: Mods = Mods(8);
    /// Canonical order and spelling in input names.
    pub const NAMES: [(Mods, &'static str); 4] = [
        (Mods::CTRL, "Ctrl"),
        (Mods::SHIFT, "Shift"),
        (Mods::ALT, "Alt"),
        (Mods::SUPER, "Super"),
    ];

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn contains(self, other: Mods) -> bool {
        self.0 & other.0 == other.0
    }

    #[must_use]
    pub fn with(self, other: Mods) -> Mods {
        Mods(self.0 | other.0)
    }

    /// The modifier a modifier key sets (`ControlLeft` → Ctrl).
    pub fn of_key(base: &str) -> Option<Mods> {
        Some(match base {
            "ControlLeft" | "ControlRight" => Mods::CTRL,
            "ShiftLeft" | "ShiftRight" => Mods::SHIFT,
            "AltLeft" | "AltRight" => Mods::ALT,
            "SuperLeft" | "SuperRight" => Mods::SUPER,
            _ => return None,
        })
    }

    pub fn labels(self) -> impl Iterator<Item = &'static str> {
        Self::NAMES
            .into_iter()
            .filter(move |(m, _)| self.contains(*m))
            .map(|(_, label)| label)
    }
}

/// A parsed input name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Input<'a> {
    pub kind: InputKind,
    pub mods: Mods,
    /// The engine's variant name (`Enter`, `South`, `Left`); empty for
    /// `mouse_motion`, `mouse_wheel` and `any_key`.
    pub base: &'a str,
}

impl<'a> Input<'a> {
    /// Parses the grammar only (prefix, modifiers, a non-empty base). Modifier
    /// names are case-insensitive; [`Input::canonical`] spells them `Ctrl+`.
    pub fn parse(name: &'a str) -> Result<Self, String> {
        let unit = |kind| Input {
            kind,
            mods: Mods::default(),
            base: "",
        };
        match name {
            "mouse_motion" => return Ok(unit(InputKind::MouseMotion)),
            "mouse_wheel" => return Ok(unit(InputKind::MouseWheel)),
            "any_key" => return Ok(unit(InputKind::AnyKey)),
            _ => {}
        }
        let (prefix, rest) = name
            .split_once(':')
            .ok_or_else(|| format!("`{name}` has no `kind:` prefix"))?;
        let kind = match prefix {
            "key" => InputKind::Key,
            "mouse" => InputKind::Mouse,
            "pad" => InputKind::Pad,
            "axis" => InputKind::Axis,
            "stick" => InputKind::Stick,
            _ => {
                return Err(format!(
                    "unknown input kind `{prefix}` in `{name}` (key, mouse, pad, axis, stick)"
                ));
            }
        };
        let mut parts: Vec<&str> = rest.split('+').collect();
        let base = parts.pop().unwrap_or_default();
        if base.is_empty() {
            return Err(format!("`{name}` names no input"));
        }
        let mut mods = Mods::default();
        for part in parts {
            let (m, _) = Mods::NAMES
                .iter()
                .find(|(_, label)| label.eq_ignore_ascii_case(part))
                .ok_or_else(|| format!("`{part}` in `{name}` is not Ctrl, Shift, Alt or Super"))?;
            mods = mods.with(*m);
        }
        if !mods.is_empty() && !matches!(kind, InputKind::Key | InputKind::Mouse) {
            return Err(format!("`{name}` can't have modifiers"));
        }
        Ok(Input { kind, mods, base })
    }

    pub fn device(&self) -> Device {
        self.kind.device()
    }

    /// The canonical name (modifiers in `Ctrl+Shift+Alt+Super` order).
    pub fn canonical(&self) -> String {
        if self.base.is_empty() {
            return self.kind.prefix().to_owned();
        }
        let mut s = format!("{}:", self.kind.prefix());
        for label in self.mods.labels() {
            s.push_str(label);
            s.push('+');
        }
        s.push_str(self.base);
        s
    }

    /// The modifier this input is, if it is a bare modifier key.
    pub fn modifier(&self) -> Option<Mods> {
        (self.kind == InputKind::Key && self.mods.is_empty())
            .then(|| Mods::of_key(self.base))
            .flatten()
    }
}

/// How two bindings collide.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overlap {
    /// The same input.
    Same,
    /// One fires whenever the other does: `key:KeyS` also fires on
    /// `key:Ctrl+KeyS` (a binding without modifiers ignores held modifiers,
    /// as in `bevy_enhanced_input`), `any_key` on every key, `stick:Left` on
    /// `axis:LeftStickX`.
    Partial,
}

/// Whether two input names collide. Unparseable names never collide.
pub fn overlap(a: &str, b: &str) -> Option<Overlap> {
    let (Ok(x), Ok(y)) = (Input::parse(a), Input::parse(b)) else {
        return None;
    };
    if x.kind == y.kind && x.base == y.base && x.mods == y.mods {
        return Some(Overlap::Same);
    }
    let partial = match (x.kind, y.kind) {
        (InputKind::Key, InputKind::Key) | (InputKind::Mouse, InputKind::Mouse) => {
            x.base == y.base && (x.mods.contains(y.mods) || y.mods.contains(x.mods))
        }
        (InputKind::AnyKey, InputKind::Key) | (InputKind::Key, InputKind::AnyKey) => true,
        (InputKind::Stick, InputKind::Axis) => stick_has_axis(x.base, y.base),
        (InputKind::Axis, InputKind::Stick) => stick_has_axis(y.base, x.base),
        _ => false,
    };
    partial.then_some(Overlap::Partial)
}

fn stick_has_axis(stick: &str, axis: &str) -> bool {
    axis.strip_prefix(stick)
        .is_some_and(|rest| rest == "StickX" || rest == "StickY")
}

/// Gamepad family, for glyph labels and icon ids. Bevy's gamepad buttons are
/// positional (`South` is the bottom face button), so the same binding reads
/// A, Cross or B depending on the pad.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PadFamily {
    /// Xbox and every XInput-style pad (the PC convention for unknown pads).
    #[default]
    Xbox,
    PlayStation,
    /// Switch layout (A east, B south).
    Nintendo,
}

impl PadFamily {
    /// From the USB vendor id the OS reports (Sony 0x054C, Nintendo 0x057E);
    /// anything else reads as Xbox.
    pub fn from_vendor_id(vendor: u16) -> Self {
        match vendor {
            0x054C => PadFamily::PlayStation,
            0x057E => PadFamily::Nintendo,
            _ => PadFamily::Xbox,
        }
    }

    fn id(self) -> &'static str {
        match self {
            PadFamily::Xbox => "xbox",
            PadFamily::PlayStation => "ps",
            PadFamily::Nintendo => "switch",
        }
    }
}

/// What to show for a binding: a text label (English fallback) and icon ids,
/// one per part (modifiers first, then the input). Icon ids are stable and
/// Fluent-id safe (`pad-ps-South`, `mod-Ctrl`, `key-KeyS`), so a content pack
/// maps them to atlas images (U3) and a locale can reword a label through a
/// `glyph-<icon id>` message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Glyph {
    pub label: String,
    pub icons: Vec<String>,
}

/// Glyph of an input name, for the given pad family. `None` for names that
/// don't parse.
pub fn glyph(name: &str, family: PadFamily) -> Option<Glyph> {
    let input = Input::parse(name).ok()?;
    let mut icons: Vec<String> = input.mods.labels().map(|m| format!("mod-{m}")).collect();
    let mut label = String::new();
    for m in input.mods.labels() {
        label.push_str(m);
        label.push('+');
    }
    let base = input.base;
    let (text, icon) = match input.kind {
        InputKind::Key => (key_label(base), format!("key-{base}")),
        InputKind::Mouse => (format!("Mouse {base}"), format!("mouse-{base}")),
        InputKind::Pad => (
            pad_label(base, family).map_or_else(|| base.to_owned(), str::to_owned),
            format!("pad-{}-{base}", family.id()),
        ),
        InputKind::Axis => (
            axis_label(base).to_owned(),
            format!("axis-{}-{base}", family.id()),
        ),
        InputKind::Stick => (
            format!("{base} Stick"),
            format!("stick-{}-{base}", family.id()),
        ),
        InputKind::MouseMotion => ("Mouse".to_owned(), "mouse-motion".to_owned()),
        InputKind::MouseWheel => ("Mouse Wheel".to_owned(), "mouse-wheel".to_owned()),
        InputKind::AnyKey => ("Any Key".to_owned(), "any-key".to_owned()),
    };
    label.push_str(&text);
    icons.push(icon);
    Some(Glyph { label, icons })
}

fn key_label(base: &str) -> String {
    let fixed = match base {
        "Escape" => "Esc",
        "ArrowUp" => "Up",
        "ArrowDown" => "Down",
        "ArrowLeft" => "Left",
        "ArrowRight" => "Right",
        "ControlLeft" => "Left Ctrl",
        "ControlRight" => "Right Ctrl",
        "ShiftLeft" => "Left Shift",
        "ShiftRight" => "Right Shift",
        "AltLeft" => "Left Alt",
        "AltRight" => "Right Alt",
        "SuperLeft" => "Left Super",
        "SuperRight" => "Right Super",
        _ => {
            return base
                .strip_prefix("Key")
                .or_else(|| base.strip_prefix("Digit"))
                .filter(|s| !s.is_empty())
                .unwrap_or(base)
                .to_owned();
        }
    };
    fixed.to_owned()
}

fn pad_label(base: &str, family: PadFamily) -> Option<&'static str> {
    use PadFamily as F;
    Some(match (base, family) {
        ("South", F::Xbox) | ("East", F::Nintendo) => "A",
        ("East", F::Xbox) | ("South", F::Nintendo) => "B",
        ("West", F::Xbox) | ("North", F::Nintendo) => "X",
        ("North", F::Xbox) | ("West", F::Nintendo) => "Y",
        ("South", F::PlayStation) => "Cross",
        ("East", F::PlayStation) => "Circle",
        ("West", F::PlayStation) => "Square",
        ("North", F::PlayStation) => "Triangle",
        ("LeftTrigger", F::Xbox) => "LB",
        ("RightTrigger", F::Xbox) => "RB",
        ("LeftTrigger2", F::Xbox) => "LT",
        ("RightTrigger2", F::Xbox) => "RT",
        ("LeftTrigger", F::PlayStation) => "L1",
        ("RightTrigger", F::PlayStation) => "R1",
        ("LeftTrigger2", F::PlayStation) => "L2",
        ("RightTrigger2", F::PlayStation) => "R2",
        ("LeftTrigger", F::Nintendo) => "L",
        ("RightTrigger", F::Nintendo) => "R",
        ("LeftTrigger2", F::Nintendo) => "ZL",
        ("RightTrigger2", F::Nintendo) => "ZR",
        ("Select", F::Xbox) => "View",
        ("Start", F::Xbox) => "Menu",
        ("Select", F::PlayStation) => "Create",
        ("Start", F::PlayStation) => "Options",
        ("Select", F::Nintendo) => "−",
        ("Start", F::Nintendo) => "+",
        ("Mode", F::Xbox) => "Xbox",
        ("Mode", F::PlayStation) => "PS",
        ("Mode", F::Nintendo) => "Home",
        ("LeftThumb", F::PlayStation) => "L3",
        ("RightThumb", F::PlayStation) => "R3",
        ("LeftThumb", _) => "LS",
        ("RightThumb", _) => "RS",
        ("DPadUp", _) => "D-pad Up",
        ("DPadDown", _) => "D-pad Down",
        ("DPadLeft", _) => "D-pad Left",
        ("DPadRight", _) => "D-pad Right",
        _ => return None,
    })
}

fn axis_label(base: &str) -> &str {
    match base {
        "LeftStickX" | "LeftStickY" => "Left Stick",
        "RightStickX" | "RightStickY" => "Right Stick",
        other => other,
    }
}
