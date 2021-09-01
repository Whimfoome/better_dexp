// 1160 - Dash To Panel
// 3628 - ArcMenu
// 2087 - Desktop Icons NG
// 2890 - Tray Icons Reloaded
// https://extensions.gnome.org

use std::process::Command;
mod download;
use download::*;

enum ExtID {
    DashToPanel = 1160,
    ArcMenu = 3628,
    DeskIconsNG = 2087,
    TrayIcons = 2890,
}

pub fn main() {
    let shell_version = get_shell_version();

    download_extension(ExtID::DashToPanel as i32, &shell_version);
    download_extension(ExtID::ArcMenu as i32, &shell_version);
    download_extension(ExtID::DeskIconsNG as i32, &shell_version);
    download_extension(ExtID::TrayIcons as i32, &shell_version);
}

fn get_shell_version() -> String {
    let shell_version = Command::new("gnome-shell").arg("--version").output().expect("Couldn't get Shell Version");
    let shell_version = shell_version.stdout;
    let shell_version = String::from_utf8(shell_version).unwrap();
    let mut shell_version = shell_version.chars();

    let mut i = 0;
    while i < 12 {
        shell_version.next();
        i += 1;
    }

    let shell_version = shell_version.as_str();

    return shell_version.to_owned();
}