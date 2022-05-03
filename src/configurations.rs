use config::Config;
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
    // let mut settings = config::Config::default();
    //
    // // add configuration values from file named configuration
    // settings.merge(config::File::with_name("configuration"))?;
    //
    // // Try to convert the configuration values into Settings type
    // settings.try_into()
    let settings = Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()
        .unwrap();

    settings.try_deserialize()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database_name)
    }

    pub fn connection_string_without_db(&self) -> String {
        format!("postgres://{}:{}@{}:{}",
                self.username, self.password, self.host, self.port)
    }
}
