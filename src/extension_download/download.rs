use std::process::Command;
use serde_json::Value;

pub fn download_extension(ext_id: i32, shell_version: &str) {
    let mut build_string = String::from("https://extensions.gnome.org/extension-info/?pk=");
    build_string.push_str(&ext_id.to_string());
    build_string.push_str("&shell_version=");
    build_string.push_str(shell_version);

    match return_json(&build_string) {
        Ok(json) => {
            let json_string: String = json["download_url"].to_string();
            
            // Trim the Quotes
            let mut json_string = json_string.chars();
            json_string.next();
            json_string.next_back();
            let json_string = json_string.as_str();

            let mut download_link = String::from("https://extensions.gnome.org");
            download_link.push_str(&json_string);

            Command::new("wget")
                .arg(download_link)
                .spawn()
                .expect("Failed");
            
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