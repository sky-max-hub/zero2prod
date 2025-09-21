#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: String,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{user}:{password}@{host}:{port}/{database}",
            host = self.host,
            port = self.port,
            user = self.user,
            password = self.password,
            database = self.database
        )
    }
}
