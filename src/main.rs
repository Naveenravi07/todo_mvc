mod templates;

use axum::{extract::State, routing::get, Form, Router};
use libsql_client::{Client, Config};
use std::sync::{Arc, Mutex};
use templates::{Addtodo, CreateTodo, Index};
use tower_http::services::ServeDir;

async fn index() -> Index {
    Index {}
}

async fn addtodo() -> Addtodo {
    Addtodo {}
}

async fn createtodo(
    State(state): State<Arc<Mutex<AppState>>>,
    Form(data): Form<CreateTodo>,
    ) -> axum::response::Html<String> {

    println!("{:?}", data);
    axum::response::Html(format!(
            r#"
            <!doctype html>
            <html>
            <head>
            <title>Book</title>
            </head>
            <body>
            <h1>Book</h1>
            <!-- Add your form and other HTML content here -->
            </body>
            </html>
            "#
            ))
}

pub struct AppState {
    pub database: Client,
}

#[tokio::main]
async fn main() {
    const DB_PATH: &str = "file:///home/shastri/deez/todo_mvc/todo.db";
    let config = Config::new(DB_PATH).expect("Failed to create database configuration");

    let db = libsql_client::Client::from_config(config)
        .await
        .expect("Failed to connect to the database");
    db.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, todo TEXT, completed BOOLEAN, priority TEXT, member TEXT)").await.expect("Failed to create todos table");

    let app_state = Arc::new(Mutex::new(AppState { database: db }));

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index))
        .route("/todo/add", get(addtodo).post(createtodo))
        .with_state(Arc::clone(&app_state));

    axum::Server::bind(&"0.0.0.0:42069".parse().expect("Invalid address"))
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}

