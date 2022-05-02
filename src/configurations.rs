use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // initialize configuration reader
    let mut settings = config::Config::default();

    // add configuration values from file named configuration
    settings.merge(config::File::with_name("configuration"))?;

    // Try to convert the configuration values into Settings type
    settings.try_into()
}
