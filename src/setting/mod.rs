use std::collections::HashMap;
use std::process;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Key {
    Spawn {
        command: String
    },

}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    keys: HashMap<String, Key>,
    background: String,
}

impl Settings {
    pub fn from_config(config: config::Config) -> Settings {
        match config.try_into() {
            Ok(setting) => {
                setting
            }
            Err(e) => {
                // TODO: Handle Error
                println!("{}", e);
                process::exit(0)
            }
        }
    }
    pub fn get_background(&self) -> &str {
        &self.background
    }
}