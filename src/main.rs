use std::net::{SocketAddr};
use serde::{Deserialize, Serialize};
use axum::{middleware, Router, ServiceExt};
use axum::extract::{Path, Query};

use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use tower_cookies::CookieManagerLayer;
use tracing;
use tower_http::services::ServeDir;
use crate::model::ModelController;
pub use self::error::{ Result};
mod error;
mod model;
mod web;


//TODO: cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
//TODO: cargo watch -q -c -w src/main.rs -x run

#[allow(unused)]
#[tokio::main]
async fn main() -> Result<()> {

    //++Server

    let controller = ModelController::new().await?;

    let api_routes = web::ticket_routes::rest_routes(controller.clone())
        .await.route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));



    let all_routes = Router::new()
        .merge(routes_to_greet())
        .merge(web::login_routes::routes())
        .nest("/api", api_routes)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(static_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("->> LISTENING TO {addr}\n");

    axum::Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();


    Ok(())

    //--Server

}
//++Routes


async fn main_response_mapper(res: Response) -> Response {

    println!("->> {:<12} - main_response_mapper ", "RESP_MAPPER");
    println!();
    res

}


fn static_routes() -> Router {

    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_to_greet() -> Router {

    Router::new()
        .route("/hello", get(handle_hello)
    )
        .route("/hello2/:name", get(handle_hello2))


}
//--Routes

// ++Handler Hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}


//http://localhost:8080/hello?name=Viktor
async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {

    println!("->> {:<12} - handle_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("<h1>Hello, {name}!</h1>"))

}


//http://localhost:8080/hello/Viktor
async fn handle_hello2(Path(name): Path<String>) -> impl IntoResponse {

    println!("->> {:<12} - handle_hello2 - {name:?}", "HANDLER");

    Html(format!("<h1>Hello, {name}!</h1>"))

}
//--Handler Hello