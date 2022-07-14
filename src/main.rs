use std::env;
use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    //find our spotify application api
    let spotmefy_app_client_id: &'static str = env!("SPOTMEFY_APP_CLIENT_ID", "Client ID is not defined in your $PATH");
    let spotmefy_app_client_secret: &'static str = env!("SPOTMEFY_APP_CLIENT_SECRET", "Client SECRET is not defined in your $PATH");

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/spotify", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
