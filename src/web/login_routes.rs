use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use crate::error::{Error, Result};
use crate::web;


pub fn routes() -> Router {
    Router::new().route("/api/login", post(login_api))
}

async fn login_api(cookie: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - login_api ", "HANDLER");

    //TODO:impl for db/auth
    // Db/auth is hardcoded for testing

    if payload.username != "Viktor" || payload.pw != "test" {
       return Err(Error::LoginError)
    }

    //++Cookies
    //TODO: Implement a real auth/sign token
    cookie.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    //--Cookies

    //Create the body on successful login request

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}


#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    pw: String,
}