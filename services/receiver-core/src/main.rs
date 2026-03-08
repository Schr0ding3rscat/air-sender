use std::{env, net::SocketAddr};

use receiver_core::serve;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let bind = env::var("AIR_SENDER_BIND").unwrap_or_else(|_| "127.0.0.1:9760".to_string());
    let token = env::var("AIR_SENDER_API_TOKEN").unwrap_or_else(|_| "dev-token".to_string());
    let addr: SocketAddr = bind.parse().expect("AIR_SENDER_BIND must be host:port");

    tracing::info!(%addr, "starting receiver-core");
    serve(addr, token).await;
}
