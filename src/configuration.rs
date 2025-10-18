use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: Application,
    pub database: DatabaseSettings,
    pub email: EmailSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailSettings {
    pub bearer_token: Secret<String>,
    pub base_url: String,
    pub from_email: String,
    pub from_email_name: String,
    pub timeout_milliseconds: u64,
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
    pub password: Secret<String>,
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

impl EmailSettings {
    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_milliseconds)
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{user}:{password}@{host}:{port}/{database}",
            host = self.host,
            port = self.port,
            user = self.user,
            password = self.password.expose_secret(),
            database = self.database
        ))
    }
}
