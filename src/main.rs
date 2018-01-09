#[macro_use]
extern crate native_windows_gui as nwg;

use nwg::{Event, Ui, FileDialog, simple_message, fatal_message, dispatch_events};

#[derive(Debug, Clone, Hash)]
pub enum AppId {
    // Controls
    MainWindow
}

// use AppId::*;

nwg_template!(
    head: setup_ui<AppId>,
    controls: [
        (AppId::MainWindow, nwg_window!(title = "PeC - PE Characteristics"; size = (280, 105)))
    ];
    events: [

    ];
    resources: [

    ];
    values: []
);

fn main() {
    let app: Ui<AppId>;

    match Ui::new() {
        Ok(_app) => { app = _app; },
        Err(e) => { fatal_message("Fatal error", &format!("{:?}", e)); }
    }

    if let Err(e) = setup_ui(&app) {
        fatal_message("Fatal error", &format!("{:?}", e));
    }

    dispatch_events();
}
