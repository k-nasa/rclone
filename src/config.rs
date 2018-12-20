use failure::Error;
use serde_derive::{Deserialize, Serialize};
use std::fs::*;
use std::io::prelude::*;
use std::path::*;
use std::str::from_utf8;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub dir: Option<String>,
}

impl Config {
    pub fn load_config() -> Result<Config, Error> {
        let mut file = Self::load_or_create_config_file()?;

        let mut buf = vec![];
        file.read_to_end(&mut buf).expect("faild load config file");
        let toml_str = from_utf8(&buf)?;

        let config: Config = if toml_str.is_empty() {
            let config = Config::default();
            let toml_str = toml::to_string(&config).unwrap();

            match file.write_all(toml_str.as_bytes()) {
                Ok(_) => config,
                Err(e) => panic!(e),
            }
        } else {
            toml::from_str(toml_str)?
        };

        Ok(config)
    }

    fn load_or_create_config_file() -> Result<File, Error> {
        let dir = Path::new(&home_dir_string()).join(".config/clone");

        DirBuilder::new().recursive(true).create(dir.clone())?;

        let filepath = &dir.join("config.toml");

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .read(true)
            .open(filepath)?;

        Ok(file)
    }
}

impl Default for Config {
    fn default() -> Self {
        let dir = Some(home_dir_string() + "/.config/clone/repos");
        Config { dir }
    }
}

fn home_dir_string() -> String {
    match dirs::home_dir() {
        Some(dir) => dir.to_str().unwrap().to_string(),
        _ => panic!("Home directory is not set"),
    }
}
