use std::process::Command;
// r#""# - Raw String

pub fn apply_dash_to_dock() {
    // Taskbar Structure
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/dash-to-panel/panel-element-positions")
        .arg(r#"'{\"0\":[{\"element\":\"showAppsButton\",\"visible\":false,\"position\":\"stackedTL\"},{\"element\":\"activitiesButton\",\"visible\":false,\"position\":\"stackedTL\"},{\"element\":\"leftBox\",\"visible\":true,\"position\":\"stackedTL\"},{\"element\":\"taskbar\",\"visible\":true,\"position\":\"centerMonitor\"},{\"element\":\"centerBox\",\"visible\":true,\"position\":\"stackedBR\"},{\"element\":\"rightBox\",\"visible\":true,\"position\":\"stackedBR\"},{\"element\":\"dateMenu\",\"visible\":true,\"position\":\"stackedBR\"},{\"element\":\"systemMenu\",\"visible\":true,\"position\":\"stackedBR\"},{\"element\":\"desktopButton\",\"visible\":true,\"position\":\"stackedBR\"}]}'"#)
        .spawn()
        .expect("Failed");
    
    // Disable show overview on startup
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/dash-to-panel/hide-overview-on-startup")
        .arg("true")
        .spawn()
        .expect("Failed");
    
    // Panel Opacity
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/dash-to-panel/trans-use-custom-opacity")
        .arg("true")
        .spawn()
        .expect("Failed");
    
    // Dynamic Panel Opacity
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/dash-to-panel/trans-use-dynamic-opacity")
        .arg("true")
        .spawn()
        .expect("Failed");
    
    // Panel Gradient
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/dash-to-panel/trans-use-custom-gradient")
        .arg("true")
        .spawn()
        .expect("Failed");
    
    // Tray Item Padding
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/dash-to-panel/tray-padding")
        .arg("4")
        .spawn()
        .expect("Failed");
    
    // Status Icon Padding
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/dash-to-panel/status-icon-padding")
        .arg("4")
        .spawn()
        .expect("Failed");
    
    // Lock Taskbar
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/dash-to-panel/taskbar-locked")
        .arg("true")
        .spawn()
        .expect("Failed");
}

pub fn apply_arc_menu() {
    // Menu Layout
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/arcmenu/menu-layout")
        .arg(r#"'Eleven'"#)
        .spawn()
        .expect("Failed");

    // Menu Placement
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/arcmenu/arc-menu-placement")
        .arg(r#"'DTP'"#)
        .spawn()
        .expect("Failed");

    // Menu Hotkey
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/arcmenu/menu-hotkey")
        .arg(r#"'Super_L'"#)
        .spawn()
        .expect("Failed");

    // Button Appearance Size
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/arcmenu/custom-menu-button-icon-size")
        .arg("34.0")
        .spawn()
        .expect("Failed");
    
    // Button Appearance Icon
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/arcmenu/distro-icon")
        .arg("6")
        .spawn()
        .expect("Failed");

    // Button Appearance Catergory
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/arcmenu/menu-button-icon")
        .arg(r#"'Distro_Icon'"#)
        .spawn()
        .expect("Failed");
}

pub fn apply_ding() {
    // Desktop Icons Don't Show Home
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/ding/show-home")
        .arg("false")
        .spawn()
        .expect("Failed");

    // Desktop Icons Don't Show Trash
    Command::new("dconf")
        .arg("write")
        .arg("/org/gnome/shell/extensions/ding/show-trash")
        .arg("false")
        .spawn()
        .expect("Failed");
}