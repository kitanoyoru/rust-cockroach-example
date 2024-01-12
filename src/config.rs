use dotenv::dotenv;

use serde::Deserialize;

use crate::database::DatabaseConnection;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConnection,
    pub database_url: String,

    pub server: String,
    pub log_level: String,
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn get_config() -> Config {
    dotenv().ok();

    match envy::from_env::<Config> {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}
