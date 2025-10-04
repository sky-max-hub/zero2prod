use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("配置失败");
    let address = format!("127.0.0.1:{}", configuration.application.address());
    let listener = TcpListener::bind(address).expect("启动失败");
    startup::run(listener)?.await
}
