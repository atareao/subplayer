use serde::{Serialize, Deserialize};

use dirs::config_dir;
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new() -> Config{
        Config { url: "".to_string(), username: "".to_string(),
                 password: "".to_string() }
    }

    fn check_config_file() -> PathBuf{
        let main_config_dir = config_dir().unwrap();
        if !main_config_dir.exists(){
            fs::create_dir(&main_config_dir);
        }
        let app_config_dir = &main_config_dir.join("subplayer");
        if !app_config_dir.exists(){
            fs::create_dir(&app_config_dir);
        }
        let app_config_file = &app_config_dir.join("subplayer.conf");
        if !app_config_file.exists(){
            let config = Config::new();
            let json = serde_json::to_string_pretty(&config).unwrap();
            fs::write(app_config_file, json);
        }
        app_config_file.to_path_buf()
    }

    pub fn load_config() -> Config{
        let app_config_file = Config::check_config_file();
        let data = fs::read_to_string(app_config_file).unwrap();
        let config: Config = serde_json::from_str(&data).unwrap();
        config
    }

    pub fn save(self){
        let app_config_file = Config::check_config_file();
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(app_config_file, json);
    }
}

