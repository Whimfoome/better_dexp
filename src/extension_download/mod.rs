// 1160 - Dash To Panel
// 3628 - ArcMenu
// 2087 - Desktop Icons NG
// 2890 - Tray Icons Reloaded
// https://extensions.gnome.org

use serde_json::Value;


pub fn main() {
    let mut hello = String::from("https://extensions.gnome.org/extension-info/?pk=");
    hello.push_str("1160");
    hello.push_str("&shell_version=");
    hello.push_str("40");

    println!("{}", hello);

    match return_json(&hello) {
        Ok(json) => {
            let json_string: String = json["download_url"].to_string();
            
            // Trim the Quotes
            let mut json_string = json_string.chars();
            json_string.next();
            json_string.next_back();
            let json_string = json_string.as_str();

            let mut download_link = String::from("https://extensions.gnome.org");
            download_link.push_str(json_string);

            println!("{}", download_link);
        },
        Err(_) => println!("Error"),
    }
}

fn return_html(url: &str) -> Result<String, ureq::Error> {
    let html_str: String = ureq::get(url).call()?.into_string()?;

    Ok(html_str)
}

fn return_json(url: &str) -> serde_json::Result<Value> {
    let my_json: Value = serde_json::from_str(&return_html(url).unwrap()).unwrap();
    Ok(my_json)
}