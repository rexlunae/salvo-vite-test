use salvo::prelude::*;
use salvo::oapi::extract::*;
use salvo::logging::Logger;
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

    let router = Router::new()
        .push(
            Router::with_path("/hello").get(hello)
        );

    let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);
    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));

    // The frontend route will process anything not previously captured, so it has to be added last.
    let router = router
        .push(
            Router::with_path("<**path>").get(
                StaticDir::new([
                    "frontend/dist"
                ])
                .defaults("index.html")
                .auto_list(true),

            )
        );

    let service = Service::new(router).hoop(Logger::new());



    //let router = Router::new().get(hello);
    info!("Binding on {}", bind);
    let acceptor = TcpListener::new(bind).bind().await;
    Server::new(acceptor).serve(service).await;
}
