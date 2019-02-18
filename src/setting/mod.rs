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

#[derive(Serialize, Deserialize)]
#[serde(tag = "tiling_method")]
pub enum LayoutManagerSettings {
    Stack {
        border: u32,
        focus_border_color: String,
        normal_border_color: String,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "tiling_method")]
pub struct BarSettings {
    pub height: u32,
    pub font_size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    keys: HashMap<String, Key>,
    background: String,
    layout_manager: LayoutManagerSettings,
    bar: BarSettings,
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
    pub fn get_layout_manager_settings(&self) -> &LayoutManagerSettings {
        &self.layout_manager
    }
    pub fn get_bar(&self) -> &BarSettings {
        &self.bar
    }
}
