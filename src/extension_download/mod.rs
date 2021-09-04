// https://extensions.gnome.org (Extensions ID's and UUID's:)
// - 1160 - Dash To Panel - dash-to-panel@jderose9.github.com
// - 3628 - ArcMenu - arcmenu@arcmenu.com
// - 2087 - Desktop Icons NG - ding@rastersoft.com
// - 2890 - Tray Icons Reloaded - trayIconsReloaded@selfmade.pl
// - 3733 - Tiling Assistant - tiling-assistant@leleat-on-github

use std::process::Command;
use lazy_static::lazy_static;
mod download;
mod configure;
use download::*;

struct Extension {
    ext_id: i32,
    uuid: String,
}

impl Extension {
    fn new(ext_id: i32, uuid: String) -> Extension {
        Extension { ext_id: ext_id, uuid: uuid }
    }
}

lazy_static! {
    static ref DASH_TO_PANEL: Extension = Extension::new(1160, "dash-to-panel@jderose9.github.com".to_owned());
    static ref ARC_MENU: Extension = Extension::new(3628, "arcmenu@arcmenu.com".to_owned());
    static ref DESK_ICONS_NG: Extension = Extension::new(2087, "ding@rastersoft.com".to_owned());
    static ref TRAY_ICONS: Extension = Extension::new(2890, "trayIconsReloaded@selfmade.pl".to_owned());
    static ref TILING_ASSIST: Extension = Extension::new(3733, "tiling-assistant@leleat-on-github".to_owned());
}


pub fn download_install() {
    Command::new("mkdir").arg("-p").arg("download-extensions").spawn().expect("Failed to make directory");

    wait_download();

    install_extensions();
}


fn wait_download() {
    let shell_version = get_shell_version();

    let mut dash_proc = download_extension(DASH_TO_PANEL.ext_id, &shell_version);
    let mut arc_proc = download_extension(ARC_MENU.ext_id, &shell_version);
    let mut desk_proc = download_extension(DESK_ICONS_NG.ext_id, &shell_version);
    let mut tray_proc = download_extension(TRAY_ICONS.ext_id, &shell_version);
    let mut tile_proc = download_extension(TILING_ASSIST.ext_id, &shell_version);

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
    install_extension(DASH_TO_PANEL.ext_id);
    install_extension(ARC_MENU.ext_id);
    install_extension(DESK_ICONS_NG.ext_id);
    install_extension(TRAY_ICONS.ext_id);
    install_extension(TILING_ASSIST.ext_id);

    // Ask to logout
    Command::new("gnome-session-quit").output().expect("Couldn't Ask to Logout");
}

fn install_extension(ext_id: i32) {
    Command::new("gnome-extensions").arg("install").arg(format!("./download-extensions/{}.zip", ext_id)).output().expect("Couldn't Install Extension");
}


pub fn config_extensions() {
    configure::apply_dash_to_dock();
    configure::apply_arc_menu();
    configure::apply_ding();

    enable_extensions();
}

fn enable_extensions() {
    Command::new("gnome-extensions").arg("enable").arg(&DASH_TO_PANEL.uuid).output().expect("Couldn't Enable Extension");
    Command::new("gnome-extensions").arg("enable").arg(&ARC_MENU.uuid).output().expect("Couldn't Enable Extension");
    Command::new("gnome-extensions").arg("enable").arg(&DESK_ICONS_NG.uuid).output().expect("Couldn't Enable Extension");
    Command::new("gnome-extensions").arg("enable").arg(&TRAY_ICONS.uuid).output().expect("Couldn't Enable Extension");
    Command::new("gnome-extensions").arg("enable").arg(&TILING_ASSIST.uuid).output().expect("Couldn't Enable Extension");
}