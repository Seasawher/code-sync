use serde_derive::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    api_key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: MyConfig = confy::load("my-app-name", None)?;
    dbg!(cfg);
    Ok(())
}
