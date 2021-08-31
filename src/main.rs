mod frontend;
use ureq;

fn main() {
    mytest();
    frontend::main();
}

fn mytest() -> Result<(), ureq::Error> {
    let body: String = ureq::get("https://extensions.gnome.org/extension/1160/dash-to-panel/").call()?.into_string()?;
    println!("{}", body);
    Ok(())
}