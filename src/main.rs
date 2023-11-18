mod templates;
use tower_http::services::ServeDir;
use axum::{ routing::get, Router};
use templates::{Index, Addtodo};

async fn index() -> Index {
    return Index{} 
}

async fn addtodo() -> Addtodo {
    return Addtodo{} 
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index))
        .route("/todo/add", get(addtodo));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

