mod templates;
use std::sync::Arc;
use axum::{extract::State, routing::get, Form, Router, debug_handler};
use libsql_client::{Client, Config};
use tokio::sync::Mutex;
use templates::{Addtodo, CreateTodo, Index};
use tower_http::services::ServeDir;


async fn index() -> Index {
    Index {}
}

async fn addtodo() -> Addtodo {
    Addtodo {}
}

#[debug_handler]
async fn createtodo(
    State(state): State<AppState>,
    Form(data): Form<CreateTodo>,
    ) -> axum::response::Html<String>{

    println!("{:?}", data);
    
    state.database.lock().await.execute("INSERT INTO todos VALUES (1,'nuts',false,'mem','sasas')").await.unwrap();

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

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<Mutex<Client>>,
}

#[tokio::main]
async fn main() {
    const DB_PATH: &str = "file:///home/shastri/balls/todo_mvc/todo.db";
    let config = Config::new(DB_PATH).expect("Failed to create database configuration");

    let db = Arc::new(Mutex::new(libsql_client::Client::from_config(config)
        .await .expect("Failed to connect to the database")));
    db.lock().await.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, todo TEXT, completed BOOLEAN, priority TEXT, member TEXT)").await.expect("Failed to create todos table");
    db.lock().await.execute("INSERT INTO todos (id, todo, completed, priority, member) VALUES (1, 'nuts', false, 'mem', 'sasas')").await.unwrap();

    let app_state = AppState{database:db};

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index))
        .route("/todo/add", get(addtodo).post(createtodo))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:42069".parse().expect("Invalid address"))
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}

