use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        env!("CARGO_PKG_NAME").into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("配置失败");
    let address = configuration.application.address();
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("数据库连接失败");
    let listener = TcpListener::bind(address).expect("启动失败");
    startup::run(listener, connection_pool)?.await
}
