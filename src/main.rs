use axum::Router;
use axum::routing::{get, method_routing::get_service};
use axum::response::{Html, IntoResponse};
use axum::extract::{Query, Path};
use serde::Deserialize;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let routes_all = Router::new()
        .merge(routes_hello()
        .merge(web::routes_login::routes())
        .fallback_service(routes_static())
    );

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind(
        "0.0.0.0:8080"
        ).await.unwrap(
    );

    axum::serve(listener, routes_all).await.unwrap();
}

// region: --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2)
    )
}

fn routes_static() -> Router {
    Router::new()
        .nest_service("/", get_service(ServeDir::new("./")))
}

// e.g., `/hello?name=Jen`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello, <strong>{name}</strong>!"))
}

// e.g., `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello, <strong>{name}</strong>!"))

}
