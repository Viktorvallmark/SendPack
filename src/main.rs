mod user;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use serde;
use axum;
use axum::{Json, Router, ServiceExt};
use axum::http::StatusCode;
use axum::routing::{get, post};
use tracing;
use crate::user::{Role, User};


#[allow(unused_imports)]
#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();


    let app = Router::new().route("/", get( || async { "Hello, World!" }))
        .route("/make_user", post(make_user));


    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),8080);

    tracing::debug!("listening on port {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()


}


async fn make_user(Json(pwdload) : Json<User>, Json(nameload): Json<User>, Json(roleload): Json<User>) -> (StatusCode, Json<User>) {

    let user = User::new(nameload.name, pwdload.pwd, roleload.role);


    (StatusCode::CREATED, Json(user))
}
