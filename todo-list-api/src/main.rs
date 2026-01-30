mod db;

use axum::{Json, Router, response::IntoResponse, routing::post};


struct UserReg {
    name: String,
    email: String,
    password: String,
}
struct Auth {
    token: String,
}


async fn user_reg(Json(user_reg): Json<UserReg>) -> impl IntoResponse {

}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/register", post(user_reg))

    println!("Hello, world!");
}
