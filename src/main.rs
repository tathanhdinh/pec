#[macro_use]
extern crate native_windows_gui as nwg;

use nwg::{Event, Ui, FileDialog, simple_message, fatal_message, dispatch_events};

#[derive(Debug, Clone, Hash)]
pub enum AppId {
    // Controls
    MainWindow,
    PeSectionGroupBox,
    FileNameLabel,

    // Resources
    GroupBoxFont
}

// use AppId::*;

nwg_template!(
    head: setup_ui<AppId>,
    controls: [
        (AppId::MainWindow, 
         nwg_window!(title = "PeC - PE Characteristics"; 
                     position=(100, 100); 
                     size=(300, 200))),

        (AppId::FileNameLabel, 
         nwg_label!(parent = AppId::MainWindow;
                    text = "Input PE"; 
                    position = (20, 30); 
                    size = (100, 30); 
                    font = Some(AppId::GroupBoxFont))),     
                    
        (AppId::PeSectionGroupBox,
         nwg_groupbox!(parent = AppId::MainWindow;
                       text = "Sections";
                       position = (20, 80); 
                       size = (250, 80); 
                       font=Some(AppId::GroupBoxFont)))

        // (AppId::MainWindow, 
        //  nwg_window!(title = "Nwg Showcase"; 
        //              position = (100, 100); 
        //              size = (500, 400))),

        // (AppId::FileNameLabel, 
        //  nwg_label!(parent = AppId::MainWindow; 
        //             text = "Selected PE"; 
        //             position = (120, 90); 
        //             size=(200, 25))),

        // (AppId::PeSectionGroupBox, 
        //  nwg_groupbox!(parent=AppId::MainWindow; 
        //                text="Section"; 
        //                position=(20, 40); 
        //                size=(130, 80)))
    ];
    events: [
    ];
    resources: [
        (AppId::GroupBoxFont, nwg_font!(family="Segoe UI"; size=14))
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
