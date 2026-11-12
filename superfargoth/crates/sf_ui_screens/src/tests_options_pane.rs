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

/// The pane's lines as shown: label, description, reason, restart, default ("" when hidden).
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
        let hidden = ui
            .world()
            .get::<Node>(entity)
            .is_some_and(|n| n.display == Display::None);
        let text = ui.text_of(entity);
        assert_eq!(
            hidden,
            text.is_empty(),
            "a line is hidden exactly when empty"
        );
        text
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
