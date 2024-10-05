#![windows_subsystem = "windows"]

mod gui {
    pub(crate) mod gui_main;
    mod gui_logic;
    mod serialize;
}
mod config;
mod utils;

use crate::gui::gui_main::App;

use iced:: Size;

fn  main() -> iced::Result {
    iced::application("My Time Tracker", App::update, App::view)
        .theme(App::theme)
        .window_size(Size::new(500., 200.))
        .run_with(App::new)
}