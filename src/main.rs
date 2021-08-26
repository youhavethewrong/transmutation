use serde_json::from_str;
use std::fs;
use std::io;
use transmutation::{replace_clipboard, Recipe};

const CONFIG_PATH: &str = "./config.json";

fn read_config() -> Result<Vec<Recipe>, io::Error> {
    let config_content = fs::read_to_string(CONFIG_PATH)?;
    let parsed: Vec<Recipe> = from_str(&config_content)?;
    Ok(parsed)
}

fn main() -> Result<(), io::Error> {
    let recipes = read_config()?;
    replace_clipboard(recipes);
    Ok(())
}
