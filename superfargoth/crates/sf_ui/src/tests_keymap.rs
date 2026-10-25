//! Keybinds (U6): input names, glyphs, keymap conflicts and resolution,
//! resets, profile tables, the rebinding flow. Synthetic actions only.

use std::collections::BTreeMap;

use crate::*;

fn ui(action: &str) -> ActionRef {
    ActionRef::new("ui", action)
}

fn game(action: &str) -> ActionRef {
    ActionRef::new("gameplay", action)
}

fn slot(action: ActionRef, device: Device, slot: usize) -> SlotRef {
    SlotRef {
        action,
        device,
        slot,
    }
}

/// A UI context with a HUD scope, and a gameplay context live beside the HUD.
fn keymap() -> Keymap {
    let mut k = Keymap::new();
    let defs = [
        ActionDef::button("ui", "confirm", ["key:Enter", "pad:South"]).required(),
        ActionDef::button("ui", "back", ["key:Escape", "key:Backspace", "pad:East"]).required(),
        ActionDef::button("ui", "tab_next", ["key:KeyE", "pad:RightTrigger"]),
        ActionDef::button("ui", "open_menu", ["key:Escape", "pad:Start"])
            .required()
            .also_in("ui.hud"),
        ActionDef::stick("ui", "nav_stick", ["stick:Left"]),
        ActionDef::button("gameplay", "jump", ["key:Space", "pad:South"]),
        ActionDef::button("gameplay", "use", ["key:KeyE", "pad:West"]),
        ActionDef::button("gameplay", "strafe_right", ["key:KeyD"]),
        ActionDef::button("gameplay", "quicksave", ["key:ctrl+KeyS"]),
        ActionDef::stick("gameplay", "look", ["stick:Right", "mouse_motion"]).slots(1, 1),
    ];
    for def in defs {
        assert_eq!(k.register(def), Ok(vec![]));
    }
    k.allow_shared(ui("back"), ui("open_menu"));
    k.link_scopes("gameplay", "ui.hud");
    k
}

#[test]
fn input_names_parse_and_overlap() {
    let i = Input::parse("key:shift+ctrl+KeyS").unwrap();
    assert_eq!(i.canonical(), "key:Ctrl+Shift+KeyS");
    assert_eq!(i.device(), Device::Kbm);
    assert_eq!(
        Input::parse("stick:Left").unwrap().device(),
        Device::Gamepad
    );
    assert_eq!(
        Input::parse("key:ControlRight").unwrap().modifier(),
        Some(Mods::CTRL)
    );
    for bad in [
        "Enter",
        "key:",
        "key:Hyper+KeyS",
        "pad:Ctrl+South",
        "joy:South",
    ] {
        assert!(Input::parse(bad).is_err(), "{bad}");
    }
    assert_eq!(overlap("key:KeyS", "key:KeyS"), Some(Overlap::Same));
    assert_eq!(overlap("key:KeyS", "key:Ctrl+KeyS"), Some(Overlap::Partial));
    assert_eq!(
        overlap("key:Ctrl+Shift+KeyS", "key:Ctrl+KeyS"),
        Some(Overlap::Partial)
    );
    assert_eq!(overlap("key:Ctrl+KeyS", "key:Alt+KeyS"), None);
    assert_eq!(overlap("any_key", "key:KeyQ"), Some(Overlap::Partial));
    assert_eq!(
        overlap("stick:Left", "axis:LeftStickY"),
        Some(Overlap::Partial)
    );
    assert_eq!(overlap("stick:Left", "axis:RightStickY"), None);
    assert_eq!(overlap("key:KeyS", "pad:South"), None);
}

#[test]
fn glyphs_follow_the_pad_family() {
    let label = |name, family| glyph(name, family).unwrap().label;
    assert_eq!(label("pad:South", PadFamily::Xbox), "A");
    assert_eq!(label("pad:South", PadFamily::PlayStation), "Cross");
    assert_eq!(label("pad:South", PadFamily::Nintendo), "B", "positional");
    assert_eq!(label("pad:East", PadFamily::Nintendo), "A");
    assert_eq!(label("pad:Start", PadFamily::PlayStation), "Options");
    assert_eq!(label("pad:LeftThumb", PadFamily::PlayStation), "L3");
    assert_eq!(label("key:Escape", PadFamily::Xbox), "Esc");
    assert_eq!(label("key:Digit1", PadFamily::Xbox), "1");
    let chord = glyph("key:Ctrl+KeyS", PadFamily::Xbox).unwrap();
    assert_eq!(chord.label, "Ctrl+S");
    assert_eq!(chord.icons, ["mod-Ctrl", "key-KeyS"]);
    assert_eq!(
        glyph("pad:North", PadFamily::PlayStation).unwrap().icons,
        ["pad-ps-North"]
    );
    assert_eq!(PadFamily::from_vendor_id(0x054C), PadFamily::PlayStation);
    assert_eq!(PadFamily::from_vendor_id(0x057E), PadFamily::Nintendo);
    assert_eq!(PadFamily::from_vendor_id(0x2DC8), PadFamily::Xbox);

    let k = keymap();
    assert_eq!(
        k.glyph(&ui("confirm"), Device::Gamepad, PadFamily::PlayStation)
            .map(|g| g.label),
        Some("Cross".to_owned())
    );
    assert_eq!(
        k.glyph(&ui("nav_stick"), Device::Kbm, PadFamily::Xbox),
        None
    );
}

#[test]
fn registration_checks_defaults() {
    let mut k = keymap();
    assert_eq!(
        k.register(ActionDef::button("ui", "confirm", [])),
        Err(KeymapError::Duplicate(ui("confirm")))
    );
    assert!(matches!(
        k.register(ActionDef::button("ui", "x", ["stick:Left"])),
        Err(KeymapError::WrongShape { .. })
    ));
    assert!(matches!(
        k.register(ActionDef::button("ui", "y", ["pad:South", "pad:North"])),
        Err(KeymapError::NoSlot { .. })
    ));
    assert!(matches!(
        k.register(ActionDef::button("ui", "z", ["Enter"])),
        Err(KeymapError::BadInput { .. })
    ));
    // Names are stored canonical; defaults don't conflict.
    assert_eq!(k.bound(&game("quicksave"), Device::Kbm), ["key:Ctrl+KeyS"]);
    assert_eq!(k.conflicts(), vec![]);

    let rows = k.rows(Some("ui"));
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[1].label, "keybind-ui-back");
    assert_eq!(
        rows[1].slots[Device::Kbm as usize],
        [
            Some("key:Escape".to_owned()),
            Some("key:Backspace".to_owned())
        ]
    );
    assert_eq!(rows[0].slots[0], [Some("key:Enter".to_owned()), None]);
    assert!(rows[4].slots[0].is_empty(), "a stick has no keyboard slot");
    assert!(rows.iter().all(|r| !r.modified && !r.conflicted));
}

#[test]
fn engine_validator_rejects_unknown_inputs() {
    let mut k = Keymap::new();
    k.set_validator(|name| {
        if name.ends_with("Nope") {
            Err("no such input".to_owned())
        } else {
            Ok(())
        }
    });
    assert!(matches!(
        k.register(ActionDef::button("ui", "a", ["pad:Nope"])),
        Err(KeymapError::BadInput { .. })
    ));
    k.register(ActionDef::button("ui", "a", ["pad:South"]))
        .unwrap();
    assert!(
        k.propose(&slot(ui("a"), Device::Kbm, 0), "key:Nope")
            .is_err()
    );
}

#[test]
fn conflicts_are_per_scope() {
    let mut k = keymap();
    // `key:KeyE` is `tab_next` (ui) and `use` (gameplay): different scopes.
    assert_eq!(k.conflicts_of(&ui("tab_next"), "key:KeyE"), vec![]);
    // The HUD's open_menu is live beside gameplay.
    let c = k.conflicts_of(&game("jump"), "key:Escape");
    assert_eq!(c.len(), 1);
    assert_eq!(c[0].other, ui("open_menu"));
    // back and open_menu share Esc by design; back isn't in the HUD scope.
    assert_eq!(k.conflicts_of(&ui("back"), "key:Escape"), vec![]);
    // A plain key fires on its chord.
    let c = k.conflicts_of(&game("strafe_right"), "key:KeyS");
    assert_eq!(c.len(), 1);
    assert_eq!(
        (c[0].other.clone(), c[0].overlap),
        (game("quicksave"), Overlap::Partial)
    );

    // KeepBoth leaves the conflict visible on both rows.
    let p = k
        .propose(&slot(game("use"), Device::Kbm, 0), "key:Space")
        .unwrap();
    assert_eq!(p.conflicts.len(), 1);
    k.apply(&p, Resolution::KeepBoth).unwrap();
    let conflicts = k.conflicts();
    assert_eq!(conflicts.len(), 2, "{conflicts:?}");
    let rows = k.rows(Some("gameplay"));
    assert!(rows[0].conflicted && rows[1].conflicted && !rows[2].conflicted);
}

#[test]
fn swap_unbind_and_required() {
    let mut k = keymap();
    // Swap: jump takes E from use; use gets jump's old Space.
    let p = k
        .propose(&slot(game("jump"), Device::Kbm, 0), "key:KeyE")
        .unwrap();
    assert_eq!(p.previous.as_deref(), Some("key:Space"));
    k.apply(&p, Resolution::Swap).unwrap();
    assert_eq!(k.bound(&game("jump"), Device::Kbm), ["key:KeyE"]);
    assert_eq!(k.bound(&game("use"), Device::Kbm), ["key:Space"]);
    assert_eq!(k.conflicts(), vec![]);

    // Unbind on a second slot: the other action loses the input.
    let p = k
        .propose(&slot(game("jump"), Device::Kbm, 1), "key:Space")
        .unwrap();
    assert_eq!(p.previous, None);
    k.apply(&p, Resolution::Unbind).unwrap();
    assert_eq!(
        k.bound(&game("jump"), Device::Kbm),
        ["key:KeyE", "key:Space"]
    );
    assert!(k.bound(&game("use"), Device::Kbm).is_empty());

    // Stale proposals are refused.
    let p = k
        .propose(&slot(game("use"), Device::Kbm, 0), "key:KeyF")
        .unwrap();
    k.reset(&game("jump"), None);
    assert_eq!(k.apply(&p, Resolution::Swap), Err(KeymapError::Stale));

    // Taking confirm's only pad button away is refused; a swap gives it ours.
    let p = k
        .propose(&slot(game("jump"), Device::Gamepad, 0), "pad:South")
        .unwrap();
    assert!(p.conflicts.is_empty(), "different scopes");
    let mut k2 = keymap();
    let p = k2
        .propose(&slot(ui("tab_next"), Device::Gamepad, 0), "pad:South")
        .unwrap();
    assert_eq!(
        k2.apply(&p, Resolution::Unbind),
        Err(KeymapError::Required {
            action: ui("confirm"),
            device: Device::Gamepad
        })
    );
    assert_eq!(
        k2.bound(&ui("tab_next"), Device::Gamepad),
        ["pad:RightTrigger"]
    );
    k2.apply(&p, Resolution::Swap).unwrap();
    assert_eq!(
        k2.bound(&ui("confirm"), Device::Gamepad),
        ["pad:RightTrigger"]
    );
    assert_eq!(
        k2.unbind(&slot(ui("confirm"), Device::Gamepad, 0)),
        Err(KeymapError::Required {
            action: ui("confirm"),
            device: Device::Gamepad
        })
    );

    // Wrong device, wrong shape, no such slot.
    assert!(matches!(
        k2.propose(&slot(ui("confirm"), Device::Kbm, 0), "pad:North"),
        Err(KeymapError::WrongDevice { .. })
    ));
    assert!(matches!(
        k2.propose(&slot(ui("nav_stick"), Device::Gamepad, 0), "pad:North"),
        Err(KeymapError::WrongShape { .. })
    ));
    assert!(matches!(
        k2.propose(&slot(ui("confirm"), Device::Gamepad, 1), "pad:North"),
        Err(KeymapError::NoSlot { .. })
    ));
}

#[test]
fn moving_within_an_action_swaps_its_slots() {
    let mut k = keymap();
    let p = k
        .propose(&slot(ui("back"), Device::Kbm, 0), "key:Backspace")
        .unwrap();
    assert!(p.conflicts.is_empty());
    k.apply(&p, Resolution::Swap).unwrap();
    assert_eq!(
        k.bound(&ui("back"), Device::Kbm),
        ["key:Backspace", "key:Escape"]
    );
    assert!(k.unbind(&slot(ui("back"), Device::Kbm, 0)).unwrap());
    assert_eq!(k.bound(&ui("back"), Device::Kbm), ["key:Escape"]);
    assert!(!k.unbind(&slot(ui("back"), Device::Kbm, 1)).unwrap());
}

#[test]
fn resets_per_action_device_context() {
    let mut k = keymap();
    k.set(&game("jump"), Device::Kbm, &["key:KeyJ"]).unwrap();
    k.set(&game("jump"), Device::Gamepad, &["pad:North"])
        .unwrap();
    k.set(&ui("tab_next"), Device::Kbm, &["key:Tab"]).unwrap();
    assert!(k.rows(Some("gameplay"))[0].modified);

    assert!(k.reset(&game("jump"), Some(Device::Gamepad)));
    assert_eq!(k.bound(&game("jump"), Device::Kbm), ["key:KeyJ"]);
    assert_eq!(k.bound(&game("jump"), Device::Gamepad), ["pad:South"]);
    assert!(!k.reset(&game("jump"), Some(Device::Gamepad)), "no change");

    assert!(k.reset_context("gameplay", None));
    assert_eq!(k.bound(&game("jump"), Device::Kbm), ["key:Space"]);
    assert_eq!(k.bound(&ui("tab_next"), Device::Kbm), ["key:Tab"]);

    let v = k.version();
    assert!(k.reset_all(None));
    assert!(k.version() > v);
    assert!(k.to_tables().is_empty());
}

#[test]
fn profile_tables_hold_only_changed_devices() {
    let mut k = keymap();
    assert!(k.to_tables().is_empty());
    k.set(&ui("confirm"), Device::Kbm, &["key:Space", "key:Enter"])
        .unwrap();
    k.set(&game("look"), Device::Kbm, &[]).unwrap();
    let tables = k.to_tables();
    let text = toml::to_string(&tables).unwrap();
    let expected: BTreeMap<String, toml::Table> = toml::from_str(
        "[ui.confirm]\nkbm = [\"key:Space\", \"key:Enter\"]\n[gameplay.look]\nkbm = []\n",
    )
    .unwrap();
    assert_eq!(tables, expected, "{text}");

    // Restart: defaults, then the tables.
    let mut again = keymap();
    assert_eq!(again.load_tables(&tables), vec![]);
    assert_eq!(again.to_tables(), tables);
    assert_eq!(again.bound(&ui("confirm"), Device::Gamepad), ["pad:South"]);

    // Through a profile.
    let mut profile = sf_mods::Profile::default();
    assert!(k.save_to(&mut profile));
    assert!(!k.save_to(&mut profile), "unchanged");
    let mut third = keymap();
    assert!(third.load_from(&profile).is_empty());
    assert_eq!(
        third.bound(&ui("confirm"), Device::Kbm),
        ["key:Space", "key:Enter"]
    );
}

#[test]
fn profile_tables_report_and_pass_through() {
    let text = r#"
[ui]
open_menu = ["key:F10", "pad:Start"]       # U1c form: every device
tab_next = { gamepad = ["joy:North", "pad:North"], mouse = [] }
confirm = { kbm = [] }                       # required: defaults come back
back = { kbm = ["key:Escape", "key:KeyB", "key:KeyQ"] }
nav_stick = { gamepad = ["pad:South"] }

[ui.later]
kbm = ["key:KeyL"]

[gameplay]
jump = 3

[my_mod]
dodge = { kbm = ["key:AltLeft"] }
"#;
    let tables: BTreeMap<String, toml::Table> = toml::from_str(text).unwrap();
    let mut k = keymap();
    let issues = k.load_tables(&tables);
    let messages: Vec<String> = issues.iter().map(ToString::to_string).collect();
    assert_eq!(issues.len(), 6, "{messages:#?}");
    assert!(
        messages
            .iter()
            .any(|m| m.contains("unknown device `mouse`"))
    );
    assert!(messages.iter().any(|m| m.contains("defaults restored")));
    assert!(
        messages
            .iter()
            .any(|m| m.contains("3 kbm inputs for 2 slot(s)"))
    );

    assert_eq!(k.bound(&ui("open_menu"), Device::Kbm), ["key:F10"]);
    assert_eq!(k.bound(&ui("open_menu"), Device::Gamepad), ["pad:Start"]);
    assert_eq!(k.bound(&ui("tab_next"), Device::Gamepad), ["pad:North"]);
    assert_eq!(
        k.bound(&ui("tab_next"), Device::Kbm),
        ["key:KeyE"],
        "untouched"
    );
    assert_eq!(k.bound(&ui("confirm"), Device::Kbm), ["key:Enter"]);
    assert_eq!(
        k.bound(&ui("back"), Device::Kbm),
        ["key:Escape", "key:KeyB"]
    );
    assert!(k.bound(&ui("nav_stick"), Device::Gamepad).is_empty());

    // Unregistered actions are kept, and apply when they register.
    let saved = k.to_tables();
    assert!(saved["my_mod"].contains_key("dodge"));
    assert!(saved["ui"].contains_key("later"));
    assert_eq!(
        k.register(ActionDef::button("ui", "later", ["key:KeyK"])),
        Ok(vec![])
    );
    assert_eq!(k.bound(&ui("later"), Device::Kbm), ["key:KeyL"]);
    assert_eq!(k.to_tables()["my_mod"], saved["my_mod"]);
}

/// Names for one frame.
fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_owned()).collect()
}

fn step(r: &mut Rebinder, k: &mut Keymap, pressed: &[&str], held: &[&str]) -> Option<RebindEvent> {
    let (pressed, held) = (names(pressed), names(held));
    r.update(
        k,
        0.1,
        CaptureFrame {
            pressed: &pressed,
            held: &held,
        },
    )
}

#[test]
fn rebinder_arms_filters_and_drains() {
    let mut k = keymap();
    let mut r = Rebinder::default();
    let target = slot(game("jump"), Device::Kbm, 0);
    r.start(&k, target.clone()).unwrap();
    assert!(r.blocks_ui());
    // Enter that opened the capture is still held: not captured.
    assert_eq!(step(&mut r, &mut k, &[], &["key:Enter"]), None);
    assert_eq!(step(&mut r, &mut k, &[], &[]), None);
    // A pad press while rebinding a key is ignored.
    assert_eq!(step(&mut r, &mut k, &["pad:North"], &["pad:North"]), None);
    assert_eq!(r.capturing(), Some(&target));
    assert_eq!(
        step(&mut r, &mut k, &["key:KeyJ"], &["key:KeyJ"]),
        Some(RebindEvent::Applied {
            slot: target.clone(),
            input: "key:KeyJ".into()
        })
    );
    assert_eq!(k.bound(&game("jump"), Device::Kbm), ["key:KeyJ"]);
    // Blocked until J is released.
    assert!(r.blocks_ui());
    step(&mut r, &mut k, &[], &["key:KeyJ"]);
    assert!(r.blocks_ui());
    step(&mut r, &mut k, &[], &[]);
    assert!(!r.blocks_ui());
}

#[test]
fn rebinder_chords_and_modifiers() {
    let mut k = keymap();
    let mut r = Rebinder::default();
    let target = slot(game("quicksave"), Device::Kbm, 0);
    r.start(&k, target.clone()).unwrap();
    step(&mut r, &mut k, &[], &[]);
    assert_eq!(
        step(&mut r, &mut k, &["key:ControlLeft"], &["key:ControlLeft"]),
        None
    );
    assert_eq!(
        step(
            &mut r,
            &mut k,
            &["key:KeyQ"],
            &["key:ControlLeft", "key:ShiftRight", "key:KeyQ"]
        ),
        Some(RebindEvent::Applied {
            slot: target.clone(),
            input: "key:Ctrl+Shift+KeyQ".into()
        })
    );
    // A modifier pressed and released alone binds itself.
    step(&mut r, &mut k, &[], &[]);
    r.start(&k, target.clone()).unwrap();
    step(&mut r, &mut k, &[], &[]);
    step(&mut r, &mut k, &["key:AltLeft"], &["key:AltLeft"]);
    assert_eq!(
        step(&mut r, &mut k, &[], &[]),
        Some(RebindEvent::Applied {
            slot: target,
            input: "key:AltLeft".into()
        })
    );
    // A stick action takes a whole stick, not a button.
    let look = slot(game("look"), Device::Gamepad, 0);
    r.start(&k, look.clone()).unwrap();
    step(&mut r, &mut k, &[], &[]);
    assert_eq!(step(&mut r, &mut k, &["pad:South"], &["pad:South"]), None);
    assert_eq!(
        step(&mut r, &mut k, &["stick:Left"], &["stick:Left"]),
        Some(RebindEvent::Applied {
            slot: look,
            input: "stick:Left".into()
        })
    );
}

#[test]
fn rebinder_conflicts_timeouts_and_refusals() {
    let mut k = keymap();
    let mut r = Rebinder::default();
    let target = slot(game("jump"), Device::Kbm, 0);
    r.start(&k, target.clone()).unwrap();
    step(&mut r, &mut k, &[], &[]);
    let Some(RebindEvent::Conflict(p)) = step(&mut r, &mut k, &["key:KeyE"], &["key:KeyE"]) else {
        panic!("conflict expected");
    };
    assert_eq!(p.conflicts[0].other, game("use"));
    assert_eq!(r.pending(), Some(&p));
    assert!(r.blocks_ui(), "draining");
    step(&mut r, &mut k, &[], &[]);
    assert!(!r.blocks_ui(), "the dialog takes UI input");
    assert_eq!(
        r.resolve(&mut k, Resolution::Swap),
        Some(RebindEvent::Applied {
            slot: target.clone(),
            input: "key:KeyE".into()
        })
    );
    assert_eq!(k.bound(&game("use"), Device::Kbm), ["key:Space"]);

    // Timeout.
    r.start(&k, target.clone()).unwrap();
    let mut out = None;
    for _ in 0..60 {
        out = out.or(step(&mut r, &mut k, &[], &[]));
    }
    assert_eq!(out, Some(RebindEvent::Cancelled { slot: target }));
    assert!(r.capturing().is_none());

    // Taking open_menu's only pad button from the HUD scope: the swap is
    // refused when there is nothing to swap in.
    let target = slot(game("use"), Device::Gamepad, 0);
    k.unbind(&target).unwrap();
    r.start(&k, target.clone()).unwrap();
    step(&mut r, &mut k, &[], &[]);
    assert!(matches!(
        step(&mut r, &mut k, &["pad:Start"], &["pad:Start"]),
        Some(RebindEvent::Conflict(_))
    ));
    assert_eq!(
        r.resolve(&mut k, Resolution::Swap),
        Some(RebindEvent::Refused {
            slot: target,
            error: KeymapError::Required {
                action: ui("open_menu"),
                device: Device::Gamepad
            }
        })
    );
    assert_eq!(k.bound(&ui("open_menu"), Device::Gamepad), ["pad:Start"]);
}

#[test]
fn press_modes_save_load_reset_and_refuse_without_choice() {
    let mut k = keymap();
    let crouch = game("crouch");
    k.register(ActionDef::button("gameplay", "crouch", ["key:KeyC"]).press(PressMode::Hold))
        .unwrap();
    assert_eq!(k.press_mode(&crouch), Some(PressMode::Hold));
    assert_eq!(k.press_mode(&game("jump")), None);
    assert_eq!(
        k.set_press_mode(&game("jump"), PressMode::Toggle),
        Err(KeymapError::NoPressMode(game("jump")))
    );
    assert!(k.to_tables().is_empty(), "defaults save nothing");
    assert_eq!(k.set_press_mode(&crouch, PressMode::Toggle), Ok(true));
    assert_eq!(k.set_press_mode(&crouch, PressMode::Toggle), Ok(false));
    let row = k.rows(Some("gameplay")).into_iter().find(|r| r.action == crouch).unwrap();
    assert!(row.modified && row.press == Some(PressMode::Toggle));
    let tables = k.to_tables();
    assert_eq!(tables["gameplay"]["crouch"].to_string(), r#"{ press = "toggle" }"#);
    // Loads back; a bad value and a press on an action without the choice are reported.
    let mut fresh = keymap();
    fresh
        .register(ActionDef::button("gameplay", "crouch", ["key:KeyC"]).press(PressMode::Hold))
        .unwrap();
    assert!(fresh.load_tables(&tables).is_empty());
    assert_eq!(fresh.press_mode(&crouch), Some(PressMode::Toggle));
    let bad: BTreeMap<String, toml::Table> =
        toml::from_str("[gameplay]\njump = { press = \"toggle\" }\ncrouch = { press = \"tap\" }\n")
            .unwrap();
    assert_eq!(fresh.load_tables(&bad).len(), 2);
    assert_eq!(fresh.press_mode(&crouch), Some(PressMode::Hold));
    // Reset of every device restores the mode; a one-device reset keeps it.
    fresh.set_press_mode(&crouch, PressMode::Toggle).unwrap();
    assert!(!fresh.reset(&crouch, Some(Device::Kbm)));
    assert!(fresh.reset(&crouch, None));
    assert_eq!(fresh.press_mode(&crouch), Some(PressMode::Hold));
}
