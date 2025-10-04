#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: Application,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct Application {
    pub host: String,
    pub port: String,
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

impl Application {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
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
