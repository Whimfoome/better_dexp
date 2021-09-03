use std::process::Command;
use serde_json::Value;

pub fn download_extension(ext_id: i32, shell_version: &str) -> std::process::Child {
    let mut build_string = String::from("https://extensions.gnome.org/extension-info/?pk=");
    build_string.push_str(&ext_id.to_string());
    build_string.push_str("&shell_version=");
    build_string.push_str(shell_version);

    let json = return_json(&build_string).unwrap();

    let json_string: String = json["download_url"].to_string();
    
    // Trim the Quotes
    let mut json_string = json_string.chars();
    json_string.next();
    json_string.next_back();
    let json_string = json_string.as_str();

    let mut download_link = String::from("https://extensions.gnome.org");
    download_link.push_str(&json_string);

    let command = Command::new("wget")
        .arg("-q") // --quiet
        .arg("-c") // --continue Resume download if the connection lost 
        .arg("-nc") // --no-clobber Overwriting file if exists
        .arg("-O") // output
        .arg(format!("./download-extensions/{}.zip", ext_id))
        .arg(download_link)
        .spawn()
        .expect("Failed");
    
    return command;
}

fn return_html(url: &str) -> Result<String, ureq::Error> {
    let html_str: String = ureq::get(url).call()?.into_string()?;

    Ok(html_str)
}

fn return_json(url: &str) -> serde_json::Result<Value> {
    let my_json: Value = serde_json::from_str(&return_html(url).unwrap()).unwrap();
    Ok(my_json)
}