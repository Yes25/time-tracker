#![windows_subsystem = "windows"]

mod gui {
    pub(crate) mod gui_main;
    mod gui_logic;
}

use crate::gui::gui_main::AppState;

use iced::{window, Application, Settings};

fn  main() {

    let settings: Settings<()> = iced::settings::Settings {
        window: window::Settings {
            size: iced::Size::new(500.0, 200.0),
            resizable: (true),
            decorations: (true),
            ..Default::default()
        },
        ..Default::default()
    };

    let _ = AppState::run(settings);
}