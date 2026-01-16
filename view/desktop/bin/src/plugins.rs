pub mod mac_window;

use tauri::{Runtime, plugin::TauriPlugin};

pub mod plugin_log {
    use tauri_plugin_log::{Target, TargetKind, fern::colors::ColoredLevelConfig};

    use super::*;

    pub fn init<R: Runtime>() -> TauriPlugin<R> {
        // FIXME: Very weird that we can disable `reqwest` logs here but not at tracing subscribers
        // Apparently `reqwest` does not generate `tracing` logs
        // And the dispatching of `log` logs to `tracing` does not work correctly

        tauri_plugin_log::Builder::default()
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { file_name: None }),
                Target::new(TargetKind::Webview),
            ])
            .level_for("tao", log::LevelFilter::Info)
            .level_for("plugin_runtime", log::LevelFilter::Info)
            .level_for("tracing", log::LevelFilter::Warn)
            .level_for("mio", log::LevelFilter::Off)
            .level_for("reqwest", log::LevelFilter::Off)
            .level_for("keyring", log::LevelFilter::Off)
            .level_for("ureq", log::LevelFilter::Off)
            .level_for("rustls", log::LevelFilter::Off)
            .level_for("webbrowser", log::LevelFilter::Off)
            .level_for("sqlx", log::LevelFilter::Info)
            .with_colors(ColoredLevelConfig::default())
            .level(if is_dev() {
                log::LevelFilter::Trace
            } else {
                log::LevelFilter::Info
            })
            .build()
    }

    fn is_dev() -> bool {
        #[cfg(dev)]
        {
            return true;
        }
        #[cfg(not(dev))]
        {
            return false;
        }
    }
}

pub mod plugin_window_state {
    use super::*;

    pub fn init<R: Runtime>() -> TauriPlugin<R> {
        tauri_plugin_window_state::Builder::default()
            .with_denylist(&["ignored", "welcome"])
            .map_label(|label| {
                if label.starts_with(crate::OTHER_WINDOW_PREFIX) {
                    "ignored"
                } else if label.starts_with("welcome") {
                    "welcome"
                } else {
                    label
                }
            })
            .build()
    }
}
