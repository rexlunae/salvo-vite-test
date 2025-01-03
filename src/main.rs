use salvo::prelude::*;
use salvo::oapi::extract::*;
use salvo::logging::Logger;
use salvo::serve_static::StaticDir;

use log::{info};

/// A sample API endpoint.
#[endpoint]
async fn hello_api(name: QueryParam<String, false>) -> String {
    format!("Hello, {}!", name.as_deref().unwrap_or("World"))
}

#[tokio::main]
async fn main() {
    let bind = "127.0.0.1:5800";
    let swagger_ui_path = "swagger-ui";

    tracing_subscriber::fmt().init();

    let router = Router::new()
        .push(
            Router::with_path("/api/hello").get(hello_api)
        );

    info!("API documentation at http://{}/{}", bind, swagger_ui_path);
    let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);
    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router(swagger_ui_path));

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
