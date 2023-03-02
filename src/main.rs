use axum::{Server, Router, routing::get};
#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root_get));
    let server = Server::bind(&"0.0.0.0:7832".parse().unwrap())
        .serve(app.into_make_service());

    let local_addr= server.local_addr();
    println!("Listening in on {local_addr}");
    server.await.unwrap();
}


async fn root_get() -> &'static str{
    "Hi from Axum!"
}