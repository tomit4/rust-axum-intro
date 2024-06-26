use crate::{web, Error, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use axum::{Json, Router};
use axum::routing::post;
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.username != "demo1" || payload.pwd != "Welcome" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    // NOTE: This tests that the regex test on our auth token 
    // in mw_auth.rs will break when an
    // improper string is passed to the regex test
    // cookies.add(Cookie::new(web::AUTH_TOKEN, "DDDDuser-1.exp.sign"));

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
