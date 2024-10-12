use gtk::prelude::*;
use gtk::Application;
use hyprland_parser::parse_config;
use std::fs;
use std::{cell::RefCell, rc::Rc};

mod gui;

fn main() {
    let app = Application::builder()
        .application_id("nnyyxxxx.hyprgui")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let gui = Rc::new(RefCell::new(gui::ConfigGUI::new(app)));

    gui.borrow_mut().hide_config_options();

    let gui_clone = gui.clone();
    gui.borrow().open_button.connect_clicked(move |_| {
        println!("Open button clicked");
        open_config_file(gui_clone.clone());
    });

    let gui_clone = gui.clone();
    gui.borrow().save_button.connect_clicked(move |_| {
        let config = gui_clone.borrow().save_config();
        let updated_config_str = config.to_string();
        println!("Saving configuration:\n{}", updated_config_str);
    });

    gui.borrow().window.present();
}

fn open_config_file(gui: Rc<RefCell<gui::ConfigGUI>>) {
    println!("open_config_file function called");
    let gui_clone = gui.clone();
    gui.borrow().open_config_file(move |path| {
        println!("Callback executed with path: {}", path);
        let config_str = fs::read_to_string(&path).unwrap();
        let parsed_config = parse_config(&config_str);
        gui_clone.borrow_mut().load_config(&parsed_config);
        gui_clone.borrow_mut().show_config_options();
    });
    println!("open_config_file function completed");
}
