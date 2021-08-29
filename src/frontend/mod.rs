use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button};

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
    let font_but: Button = builder.object("but3").expect("Couldn't get button");

    theme_but.connect_clicked(move |_| {
        println!("Clicked Theme Button");
    });

    ext_but.connect_clicked(move |_| {
        println!("Clicked Extensions Button");
    });

    font_but.connect_clicked(move |_| {
        println!("Clicked Font Button");
    });

    window.show();
}