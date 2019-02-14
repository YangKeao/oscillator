extern crate config;
extern crate dirs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate env_logger;

mod setting;
mod x11;

use clap::App;
use config::File;
use config::Config;
use config::FileFormat;
use setting::Settings;

fn main() {
    env_logger::init();

    let _matches = App::new("Oscillator")
        .version("0.1.0")
        .about("A simple window manager")
        .author("Yang Keao")
        .get_matches();

    // TODO: Handle Error
    let mut config = Config::default();
    config
        .merge(File::with_name(
            &format!("{}/.oscillator", dirs::home_dir().unwrap().to_str().unwrap())
        ).format(FileFormat::Json)).unwrap()
        .merge(File::with_name(
            "/etc/oscillator"
        ).format(FileFormat::Json)).unwrap();

    let settings = Settings::from_config(config);

    let x11 = x11::X11::setup();
    x11.main_loop();
}
