use crate::{Error, Result};
use axum::Json;
use axum::Router;
use axum::routing::post;
use serde::{Deserialize};
use serde_json::{Value, json};

#[derive(Debug, Deserialize)]

struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }
    let body = Json(json!({
        "result":{
            "success" : true
        }
    }));
    Ok(body)
}
pub fn router_api_login() -> Router {
    Router::new().route("/api/login", post(api_login))
}

