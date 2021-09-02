use std::process::Command;

pub fn main() {
    dark_theme();
    no_hot_corner();
    minimize_titlebar();
    no_mouse_accel();
}

fn dark_theme() {
    Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.interface")
        .arg("gtk-theme")
        .arg("'Adwaita-dark'")
        .output()
        .expect("Failed");
}

fn no_hot_corner() {
    Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.interface")
        .arg("enable-hot-corners")
        .arg("false")
        .output()
        .expect("Failed");
}

fn minimize_titlebar() {
    Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.wm.preferences")
        .arg("button-layout")
        .arg("'appmenu:minimize,close'")
        .output()
        .expect("Failed");
}

fn no_mouse_accel() {
    Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.peripherals.mouse")
        .arg("accel-profile")
        .arg("'flat'")
        .output()
        .expect("Failed");
}