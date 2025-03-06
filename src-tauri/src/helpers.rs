use tauri::{Builder, Wry};

pub(crate) fn observability(mut builder: Builder<Wry>) -> Builder<Wry> {
    #[cfg(debug_assertions)]
    {
        let devtools = tauri_plugin_devtools::init();
        builder = builder.plugin(devtools);
    }

    #[cfg(not(debug_assertions))]
    {
        use tauri_plugin_log::fern::colors::ColoredLevelConfig;
        use tauri_plugin_log::{Builder, Target, TargetKind};

        let log_plugin = Builder::default()
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { file_name: None }),
                Target::new(TargetKind::Webview),
            ])
            .with_colors(ColoredLevelConfig::default())
            .build();

        builder = builder.plugin(log_plugin);
    }

    builder
}
