use axum::{routing::{get, get_service}, Router, response::{Html, IntoResponse, Response}, extract::{Query, Path}, middleware};
use serde::Deserialize;
use tower_http::services::ServeDir;
use std::{net::SocketAddr};

pub use self::error::{Error, Result};

mod error;
mod web;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(router_static());
         

// region: --- Start Server 
    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING on {addr}");

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();

// endregion: --- Start Server

}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper ", "RES_MAPPER");
    println!();
    res
}


#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

fn router_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:    --- Routers Hello

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))

}

// e.g. '/hello?name=discite'
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello, <strong>{name}</strong>"))
}

// e.g. '/hello2/discite'
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello, <strong>{name}</strong>"))
}

// endregion:    --- Routers Hello