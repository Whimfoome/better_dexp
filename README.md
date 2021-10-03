# Better GNOME Desktop Experience

Tired of having the best feeling DE with the worst defaults? This app is for you. Transform the default GNOME look to:

![Screenshot](/screenshot.png)

- Theme
  - Adwaita Dark
  - Minimize Button
  - Disabled Hot Corner
  - Disabled Mouse Acceleration
- Extensions
  - [Dash to Panel](https://github.com/home-sweet-gnome/dash-to-panel)
  - [ArcMenu](https://gitlab.com/arcmenu/ArcMenu)
  - [Desktop Icons NG](https://gitlab.com/rastersoft/desktop-icons-ng)
  - [Tray Icons Reloaded](https://github.com/MartinPL/Tray-Icons-Reloaded)
  - [Tiling Assistant](https://github.com/Leleat/Tiling-Assistant)
  - _With sensible defaults_
- [Other things I do with my Arch Setup](https://gist.github.com/Whimfoome/e033dd6bf4fc155e9ccfbdd60ef1b9a2)


## You need
- gtk4 support?
- gnome-shell
- gnome-menus* (for ArcMenu to work)
- wget

## How To Use
- Download the `.AppImage` from [Releases](https://github.com/Whimfoome/better_dexp/releases)
- Allow Execution and Open
- Click **Better Theme** and **Install Extensions**. You will be asked to Log Out, do it (this will close all your running apps!) 
- Open the app again and click **Apply Extensions Configs**

## How to Build
- You need Rust installed, recomended from [RustUp](https://www.rust-lang.org/tools/install)
- Clone this repo `git clone https://github.com/Whimfoome/better_dexp.git` && `cd better_dexp`
- `cargo build --release`
### Build with AppImage
- You need [AppImageKit](https://github.com/AppImage/AppImageKit) (`yay -S appimagetool-bin`)
- You need [Cargo-AppImage](https://github.com/StratusFearMe21/cargo-appimage) (`cargo install cargo-appimage`)
- To build it, just run `cargo appimage` in the project directory

## Credits
- Thanks to [this script by cyfrost](https://github.com/cyfrost/install-gnome-extensions), I found out how to build the download link for extensions properly :)
- Icon App from [Reversal Icon Theme](https://github.com/yeyushengfan258/Reversal-icon-theme), exact name - gnome-panel-launcher