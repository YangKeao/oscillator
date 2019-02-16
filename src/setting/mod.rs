use std::collections::HashMap;
use std::process;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Key {
    Spawn {
        command: Vec<String>
    },
    Quit,
}

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum TilingMethod {
    Stack
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    keys: HashMap<String, Key>,
    background: String,
    tiling_method: TilingMethod
}

impl Settings {
    pub fn from_config(config: config::Config) -> Settings {
        match config.try_into() {
            Ok(settings) => {
                let settings: Settings = settings; // TODO: Better way to give type annotation
                for (key, _) in settings.get_keys() {
                    info!("Map KEY: {}", key);
                }
                settings
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
    pub fn get_keys(&self) -> &HashMap<String, Key> {
        &self.keys
    }
    pub fn get_tiling_method(&self) -> TilingMethod {
        self.tiling_method
    }
}
