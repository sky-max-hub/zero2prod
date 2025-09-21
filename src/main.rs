use std::net::TcpListener;
use zero2prod::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("启动失败");
    run(listener)?.await
}
