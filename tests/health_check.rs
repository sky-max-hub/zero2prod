use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::domain::SubscriberEmail;
use zero2prod::email_client::EmailClient;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            env!("CARGO_PKG_NAME").into(),
            "trace".into(),
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            env!("CARGO_PKG_NAME").into(),
            default_filter_level,
            std::io::sink,
        );
        init_subscriber(subscriber);
    }
});

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app().await;
    let configuration = get_configuration().expect("配置失败");
    let connection_string = configuration.database.connection_string();
    let _ = PgConnection::connect(&connection_string.expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing email"),
        ("name=bogus", "missing the name"),
    ];
    for (body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(
            400,
            response.status().as_u16(),
            "Unexpected error response body {}.",
            error_message
        );
    }
}

async fn spawn_app() -> String {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0").expect("启动失败");
    let port = listener.local_addr().unwrap().port();
    let configuration = get_configuration().expect("配置失败");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("数据库连接失败");
    let email_client = EmailClient::new(configuration.email);
    let server =
        zero2prod::startup::run(listener, connection_pool, email_client).expect("启动失败");
    let _ = tokio::spawn(server);
    format!("http://localhost:{}", port)
}
