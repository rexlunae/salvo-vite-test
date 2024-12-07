use salvo::prelude::*;
use salvo::serve_static::StaticDir;

use log::{info};

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    let bind = "127.0.0.1:5800";
    tracing_subscriber::fmt().init();

    let router = Router::with_path("<**path>").get(
        StaticDir::new([
            "frontend/dist"
        ])
        .defaults("index.html")
        .auto_list(true),
    );

    //let router = Router::new().get(hello);
    info!("Binding on {}", bind);
    let acceptor = TcpListener::new(bind).bind().await;
    Server::new(acceptor).serve(router).await;
}
