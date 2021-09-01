mod frontend;
mod extension_download;
mod better_theme;

fn main() {
    extension_download::main();
    frontend::main();
}

// match return_html("https://extensions.gnome.org/extension/1160/dash-to-panel/") {
//     Ok(html) => println!("html {}", html),
//     Err(_) => println!("dsad"),
// }

// fn return_html(url: &str) -> Result<String, ureq::Error> {
//     let html_str: String = ureq::get(url).call()?.into_string()?;
//     Ok(html_str)
// }