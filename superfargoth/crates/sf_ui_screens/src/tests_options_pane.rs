//! The options screen's description pane: the focused or hovered row's label, description,
//! disabled reason, restart badge and default value.

use super::*;

/// Strings for the demo settings' pane (the demo registry ships no Fluent file).
fn localizer() -> sf_ui::Localizer {
    let source = sf_ui::L10nSource {
        mod_id: "demo".into(),
        locale: "en".parse().unwrap(),
        path: "l10n/en/demo.ftl".into(),
        text: "set-demo-difficulty = Difficulty\n\
               set-demo-difficulty-desc = How hard the fights are.\n\
               set-demo-subtitles = Subtitles\n\
               set-demo-subtitles-off = Needs a voiced language.\n"
            .into(),
    };
    sf_ui::Localizer::build(&["en".parse().unwrap()], &[source], false).0
}

fn open() -> (Ui, Entity) {
    let model = OptionsModel::from_settings(&demo::registry(), None, Some(&localizer()));
    open_options_with(model)
}

/// The pane's lines as shown: label, description, reason, restart, default ("" when empty).
fn pane(ui: &Ui, root: Entity) -> [String; 5] {
    let pane = options_parts(ui, root).pane();
    [
        pane.label,
        pane.description,
        pane.reason,
        pane.restart,
        pane.default,
    ]
    .map(|entity| {
        assert_ne!(
            ui.world().get::<Node>(entity).unwrap().display,
            Display::None,
            "an empty line keeps its place"
        );
        ui.text_of(entity)
    })
}

fn hover(ui: &mut Ui, root: Entity, key: Option<&str>) {
    let mut screen = ui.world_mut().get_mut::<OptionsScreen>(root).unwrap();
    screen.set_hovered(key.map(str::to_owned));
    ui.frames(1);
}

fn lines(lines: [&str; 5]) -> [String; 5] {
    lines.map(str::to_owned)
}

#[test]
fn options_pane_shows_the_focused_row_and_follows_the_focus() {
    let (mut ui, root) = open();
    let parts = options_parts(&ui, root);
    let master = sf_ui::audio_volume_key("master");
    assert_eq!(ui.focus(), parts.row(&master));
    assert!(ui.class_of(parts.pane().root).contains("sf-options-pane"));
    let label = ui
        .world()
        .get::<OptionsScreen>(root)
        .unwrap()
        .model
        .row(&master)
        .unwrap()
        .label
        .clone();
    assert_eq!(
        pane(&ui, root),
        lines([&label, "", "", "", "Default: 100%"]),
        "no Fluent string for a built-in's description: none shown, not its id"
    );

    show_options_page(&mut ui, root, "gameplay");
    assert_eq!(
        pane(&ui, root),
        lines([
            "Difficulty",
            "How hard the fights are.",
            "",
            "",
            "Default: 50"
        ])
    );
    ui.tap(GamepadButton::DPadDown);
    assert_eq!(
        pane(&ui, root),
        lines(["Subtitles", "", "", "", "Default: On"]),
        "the reason shows only while disabled"
    );
    // Key bindings and Back describe nothing.
    ui.tap(GamepadButton::DPadDown);
    assert_eq!(ui.focus(), Some(options_parts(&ui, root).keybinds()));
    assert_eq!(pane(&ui, root), lines(["", "", "", "", ""]));
}

#[test]
fn options_pane_shows_the_restart_badge_and_a_formatted_default() {
    let (mut ui, root) = open();
    show_options_page(&mut ui, root, "display");
    let [label, _, _, restart, default] = pane(&ui, root);
    assert_eq!(label, "Quality");
    assert_eq!(
        restart, "Applies after restart",
        "demo.quality is apply = Restart"
    );
    assert_eq!(
        default, "Default: Medium",
        "a choice's default reads as its option label"
    );
    ui.tap(GamepadButton::DPadDown);
    let [label, _, _, restart, default] = pane(&ui, root);
    assert_eq!(label, "Brightness");
    assert_eq!(restart, "", "a live setting has no badge");
    assert_eq!(default, "Default: 1.00", "as many decimals as the step");
}

#[test]
fn options_pane_shows_the_hovered_row_without_moving_focus() {
    let (mut ui, root) = open();
    show_options_page(&mut ui, root, "gameplay");
    {
        let mut screen = ui.world_mut().get_mut::<OptionsScreen>(root).unwrap();
        screen.model.set_enabled("demo.subtitles", false);
    }
    ui.frames(1);
    let difficulty = options_parts(&ui, root).row("demo.difficulty");
    assert_eq!(ui.focus(), difficulty);

    // A disabled row can't take the focus, but hovering it shows why it is disabled.
    hover(&mut ui, root, Some("demo.subtitles"));
    assert_eq!(
        pane(&ui, root),
        lines([
            "Subtitles",
            "",
            "Needs a voiced language.",
            "",
            "Default: On"
        ])
    );
    assert_eq!(ui.focus(), difficulty, "hover never moves the focus");
    assert_eq!(
        ui.world().get::<OptionsScreen>(root).unwrap().focus(),
        &OptionsFocus::Row("demo.difficulty".into())
    );

    // The pointer leaves: back to the focused row.
    hover(&mut ui, root, None);
    assert_eq!(pane(&ui, root)[0], "Difficulty");

    // A hovered key of another page is ignored; a page switch forgets the hover.
    hover(&mut ui, root, Some("demo.quality"));
    assert_eq!(pane(&ui, root)[0], "Difficulty");
    hover(&mut ui, root, Some("demo.subtitles"));
    show_options_page(&mut ui, root, "display");
    assert_eq!(
        ui.world().get::<OptionsScreen>(root).unwrap().hovered(),
        None
    );
    assert_eq!(pane(&ui, root)[0], "Quality");
}

#[test]
fn options_model_reads_descriptions_reasons_restart_and_defaults() {
    let model = OptionsModel::from_settings(&demo::registry(), None, Some(&localizer()));
    let difficulty = model.row("demo.difficulty").unwrap();
    assert_eq!(
        difficulty.description.as_deref(),
        Some("How hard the fights are.")
    );
    assert_eq!(difficulty.default, sf_ui::SettingValue::Int(50));
    assert!(!difficulty.restart);
    let subtitles = model.row("demo.subtitles").unwrap();
    assert_eq!(
        subtitles.disabled_reason.as_deref(),
        Some("Needs a voiced language.")
    );
    assert_eq!(subtitles.description, None);
    assert!(model.row("demo.quality").unwrap().restart);
    // Without a localizer, keys with no string read as none.
    let bare = OptionsModel::from_settings(&demo::registry(), None, None);
    assert_eq!(bare.row("demo.difficulty").unwrap().description, None);
    assert_eq!(bare.row("demo.subtitles").unwrap().disabled_reason, None);
}

/// The window root's laid-out rect: size and centre.
pub(super) fn window_rect(ui: &Ui, root: Entity) -> (Vec2, Vec2) {
    let size = ui.world().get::<ComputedNode>(root).unwrap().size();
    let at = ui.world().get::<UiGlobalTransform>(root).unwrap().translation;
    (size, at)
}

#[test]
fn options_window_keeps_its_frame_on_hover_value_and_page_change_and_its_rows_scroll() {
    const LONG: &str = "Very high, with every effect and the far distance turned all the way up";
    let mut model = OptionsModel::from_settings(&demo::registry(), None, Some(&localizer()));
    let display = model.pages.iter().position(|p| p.id == "display").unwrap();
    let page = &mut model.pages[display];
    let quality = page.rows.iter_mut().find(|r| r.key == "demo.quality").unwrap();
    quality.description = Some(LONG.repeat(4));
    if let OptionKind::Choice(options) = &mut quality.kind {
        options.push(("ultra".into(), LONG.into()));
    }
    // Enough rows to overflow any window: plain toggles with no description.
    let filler = OptionRow {
        key: String::new(),
        label: "Filler".into(),
        kind: OptionKind::Toggle,
        value: sf_ui::SettingValue::Bool(true),
        enabled: true,
        description: None,
        disabled_reason: None,
        restart: false,
        default: sf_ui::SettingValue::Bool(true),
        format: sf_ui::records::ValueFormat::Raw,
        reset: true,
    };
    for i in 0..60 {
        page.rows.push(OptionRow {
            key: format!("demo.filler{i}"),
            ..filler.clone()
        });
    }
    let (mut ui, root) = open_options_with(model);
    show_options_page(&mut ui, root, "display");
    ui.frames(2);
    let frame = window_rect(&ui, root);
    let viewport = TARGET.as_vec2();
    assert_eq!(frame.0, Vec2::new(960.0, 700.0), "60% x 70% of 1600 x 1000");
    assert!((frame.1 - viewport / 2.0).length() < 1.0, "centred: {}", frame.1);

    let same = |ui: &mut Ui, what: &str| {
        ui.frames(2);
        assert_eq!(window_rect(ui, root), frame, "the window moved on {what}");
    };
    hover(&mut ui, root, Some("demo.quality"));
    let description = options_parts(&ui, root).pane().description;
    assert!(
        ui.world().get::<ComputedNode>(description).unwrap().size().y > 0.0,
        "the long description is laid out (text measures headless)"
    );
    same(&mut ui, "hovering a long description");
    hover(&mut ui, root, Some("demo.filler3"));
    same(&mut ui, "hovering a row with none");
    hover(&mut ui, root, None);
    let value = options_parts(&ui, root).value("demo.quality").unwrap();
    let row = options_parts(&ui, root).row("demo.quality").unwrap();
    let row_size = ui.world().get::<ComputedNode>(row).unwrap().size();
    {
        let mut screen = ui.world_mut().get_mut::<OptionsScreen>(root).unwrap();
        let rows = &mut screen.model.pages[display].rows;
        rows.iter_mut().find(|r| r.key == "demo.quality").unwrap().value =
            sf_ui::SettingValue::Text("ultra".into());
    }
    same(&mut ui, "a value change to the longest label");
    assert_eq!(ui.text_of(value), LONG);
    assert_eq!(
        ui.world().get::<ComputedNode>(row).unwrap().size(),
        row_size,
        "the value column keeps its width and one line"
    );
    show_options_page(&mut ui, root, "gameplay");
    same(&mut ui, "a page switch");
    show_options_page(&mut ui, root, "display");
    same(&mut ui, "a page switch back");

    // The rows column scrolls: its content is taller than its box, and the wheel area is on.
    let rows = ui
        .world()
        .get::<ChildOf>(options_parts(&ui, root).row("demo.quality").unwrap())
        .unwrap()
        .parent();
    let node = ui.world().get::<ComputedNode>(rows).unwrap();
    assert!(
        node.content_size().y > node.size().y,
        "content {} fits the {} px column",
        node.content_size().y,
        node.size().y
    );
    assert_eq!(
        ui.world().get::<Node>(rows).unwrap().overflow,
        Overflow::scroll_y()
    );
    assert!(ui.world().get::<bevy_ui_widgets::ScrollArea>(rows).is_some());
}

#[test]
fn keybinds_window_opens_in_the_options_frame_and_its_rows_scroll() {
    let (ui, options) = open_options();
    let frame = window_rect(&ui, options);
    drop(ui);

    // A bare root, as the app spawns it (the screen places its own window); enough actions to
    // overflow any window.
    let mut ui = Ui::new();
    {
        let mut keymap = ui.world_mut().resource_mut::<UiKeymap>();
        for i in 0..60 {
            let _ = keymap
                .0
                .register(sf_ui::ActionDef::button("filler", &format!("a{i}"), []));
        }
    }
    let root = ui.open("keybinds", KeybindsScreen::new(None));
    ui.frames(2);
    assert_eq!(
        window_rect(&ui, root),
        frame,
        "keybinds opens in the options window's frame"
    );

    // Focus down the list: the rows scroll, the window stays put.
    let first = ui.focus().unwrap();
    for _ in 0..30 {
        ui.tap(GamepadButton::DPadDown);
    }
    assert_ne!(ui.focus(), Some(first));
    ui.frames(2);
    assert_eq!(window_rect(&ui, root), frame, "the window moved on focus");
    let row = ui.world().get::<ChildOf>(ui.focus().unwrap()).unwrap().parent();
    let rows = ui.world().get::<ChildOf>(row).unwrap().parent();
    assert!(
        ui.world().get::<ScrollPosition>(rows).unwrap().y > 0.0,
        "the list scrolled to the focused row"
    );
    let node = ui.world().get::<ComputedNode>(rows).unwrap();
    assert!(
        node.content_size().y > node.size().y,
        "content {} fits the {} px list",
        node.content_size().y,
        node.size().y
    );
    assert_eq!(
        ui.world().get::<Node>(rows).unwrap().overflow,
        Overflow::scroll_y()
    );
    assert!(ui.world().get::<bevy_ui_widgets::ScrollArea>(rows).is_some());
}
