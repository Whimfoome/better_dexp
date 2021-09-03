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
    TilingAssist = 3733,
}

pub fn main() {
    Command::new("mkdir").arg("-p").arg("download-extensions").spawn().expect("Failed");

    wait_download();

    install_extensions();
}


fn wait_download() {
    let shell_version = get_shell_version();

    let mut dash_proc = download_extension(ExtID::DashToPanel as i32, &shell_version);
    let mut arc_proc = download_extension(ExtID::ArcMenu as i32, &shell_version);
    let mut desk_proc = download_extension(ExtID::DeskIconsNG as i32, &shell_version);
    let mut tray_proc = download_extension(ExtID::TrayIcons as i32, &shell_version);
    let mut tile_proc = download_extension(ExtID::TilingAssist as i32, &shell_version);

    match dash_proc.wait() {
        Ok(status) => println!("Finished downloading DashToPanel: {}", status),
        Err(e) => print!("Error: {}", e),
    }

    match arc_proc.wait() {
        Ok(status) => println!("Finished downloading ArcMenu: {}", status),
        Err(e) => print!("Error: {}", e),
    }

    match desk_proc.wait() {
        Ok(status) => println!("Finished downloading DeskIconsNG: {}", status),
        Err(e) => print!("Error: {}", e),
    }

    match tray_proc.wait() {
        Ok(status) => println!("Finished downloading TrayIcons: {}", status),
        Err(e) => print!("Error: {}", e),
    }

    match tile_proc.wait() {
        Ok(status) => println!("Finished downloading TilingAssist: {}", status),
        Err(e) => print!("Error: {}", e),
    }
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


fn install_extensions() {
    install_extension(ExtID::DashToPanel as i32);
    install_extension(ExtID::ArcMenu as i32);
    install_extension(ExtID::DeskIconsNG as i32);
    install_extension(ExtID::TrayIcons as i32);
    install_extension(ExtID::TilingAssist as i32);

    // Ask to logout
    Command::new("gnome-session-quit").output().expect("Couldn't Ask to Logout");
}

fn install_extension(ext_id: i32) {
    Command::new("gnome-extensions").arg("install").arg(format!("./download-extensions/{}.zip", ext_id)).output().expect("Couldn't Install Extension");
}