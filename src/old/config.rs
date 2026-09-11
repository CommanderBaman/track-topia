use std::{
    error::Error,
    fs::File,
    io::{self, BufReader, BufWriter, ErrorKind},
    path::PathBuf,
};

use directories::BaseDirs;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{APP_NAME, DEFAULT_CONFIG_NAME, DEFAULT_USER_NAME},
    db::Database,
    file::make_file_if_not_exists,
    user::{User, UserDatabase},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    save_path: PathBuf,
    pub user: Option<User>,
}

impl Config {
    fn get_config_path() -> Option<PathBuf> {
        BaseDirs::new().map(|base_dirs| {
            base_dirs
                .data_local_dir()
                .join(APP_NAME)
                .join(DEFAULT_CONFIG_NAME)
        })
    }

    pub fn new() -> Result<Config, io::Error> {
        // make config file if not exists
        if let Some(save_path) = Config::get_config_path() {
            make_file_if_not_exists(&save_path).expect("unable to form config file");
            Ok(Config {
                save_path,
                user: None,
            })
        } else {
            Err(io::Error::new(
                ErrorKind::NotFound,
                "coulnd't find a place to store configuration file",
            ))
        }
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let json_file = File::create(&self.save_path).expect("config file must be present");
        let writer = BufWriter::new(json_file);
        serde_json::to_writer_pretty(writer, &self).expect("able to write the config to json file");
        Ok(())
    }

    fn update_user(&mut self, user_name: &str, user_id: &i32) {
        self.user = Some(User::new(user_id.to_owned(), user_name.to_owned()))
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        // read config
        let json_file = File::open(&self.save_path).expect("config file must be present");
        let reader = BufReader::new(json_file);
        // TODO: handle not able to read issue

        if let Ok(config) = serde_json::from_reader::<_, Config>(reader) {
            self.user = config.user;
            self.save_path = config.save_path;
        }
        Ok(())
    }
}

pub fn initialize_user(user_name: Option<&str>) -> Result<(), Box<dyn Error>> {
    // extract arguments
    let user_name = user_name
        .filter(|name| !name.is_empty())
        .unwrap_or(DEFAULT_USER_NAME)
        .to_string();

    // initialize config and database
    let mut config = Config::new().expect("configuration should be made");
    let database = Database::new().expect("database must be initialized before initializing user");

    // load config
    config.load().expect("config to be loaded");

    // if config already populated
    let mut name_to_update = String::new();
    let mut id_to_update = 0;
    if let Some(user_in_config) = &config.user {
        // load current name from db
        if let Ok(user_in_db) = database.get_user_by_name(&user_name) {
            // if user in db, then update with db
            name_to_update = user_in_db.name;
            id_to_update = user_in_db.id;
        } else if user_in_config.name == user_name {
            // if same as config, then update with config
            name_to_update = user_in_config.name.clone();
            id_to_update = user_in_config.id;
        }
    }
    if name_to_update.is_empty() {
        // if no update till now => no user present
        // make a completely new user
        let user_id = database
            .create_user(&user_name)
            .expect("create user query should work");
        name_to_update = user_name;
        id_to_update = user_id;
    }
    config.update_user(&name_to_update, &id_to_update);

    // save config
    config.save().expect("config was not able to save");
    Ok(())
}
