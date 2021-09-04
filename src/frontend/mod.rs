use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button};

use crate::better_theme as better_theme;
use crate::extension_download as extension_download;

pub fn main() {
    let application = Application::new(
        Some("org.better_dexp.whimfoome"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let ui_src = include_str!("menu.xml");
    let builder = Builder::from_string(ui_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let theme_but: Button = builder.object("but1").expect("Couldn't get button");
    let ext_but: Button = builder.object("but2").expect("Couldn't get button");
    let apply_but: Button = builder.object("but3").expect("Couldn't get button");

    theme_but.connect_clicked(move |_| {
        better_theme::main();
    });

    ext_but.connect_clicked(move |_| {
        extension_download::download_install();
    });

    apply_but.connect_clicked(move |_| {
        extension_download::config_extensions();
    });

    window.show();
}