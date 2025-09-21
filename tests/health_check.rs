use std::net::TcpListener;

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

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("启动失败");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("启动失败");
    let _ = tokio::spawn(server);
    format!("http://localhost:{}", port)
}
