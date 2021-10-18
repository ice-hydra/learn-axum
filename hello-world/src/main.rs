use axum::{Router, handler::get, response::Html};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = "0.0.0.0:8080";
    println!("Listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>你好，世界</h1>")
}
