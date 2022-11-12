use druid::{AppLauncher, WindowDesc};

mod data;
use data::AppState;

mod view;
use view::build_ui;

mod delegate;
use delegate::Delegate;

mod controllers;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Chord Progression Tester")
        .window_size((400.0, 400.0));

    // let initial_state = AppState::load_from_json();
    let initial_state = AppState::new();

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Failed to launch application")
}
