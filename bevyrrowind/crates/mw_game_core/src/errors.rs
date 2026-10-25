//! How shipped builds treat failed commands and system errors.

/// Shipped builds log a failed command or a system's returned error instead
/// of panicking (Bevy's default): a command on an entity that streaming
/// despawned the same frame (DR-4) must not end the game. Real panics keep
/// unwinding. Dev builds (the app's `dev` feature turns on this crate's) keep
/// Bevy's default so these bugs stay loud.
pub fn shipped_errors(app: &mut bevy_app::App) {
    #[cfg(not(feature = "dev"))]
    app.insert_resource(bevy_ecs::error::FallbackErrorHandler(|mut err, ctx| {
        use bevy_ecs::error::{Severity, error, match_severity};
        if let Some(payload) = err.take_payload() {
            std::panic::resume_unwind(payload);
        }
        if err.severity() == Severity::Panic { error(err, ctx) } else { match_severity(err, ctx) }
    }));
    #[cfg(feature = "dev")]
    let _ = app;
}
