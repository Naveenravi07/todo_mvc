mod templates;
use std::sync::Arc;
use libsql_client::{Config, Client};
use tower_http::services::ServeDir;
use axum::{ routing::get, Router, Form,extract::State};
use templates::{Index, Addtodo, CreateTodo};

async fn index() -> Index {
    return Index{} 
}

async fn addtodo() -> Addtodo {
    return Addtodo{} 
}

async fn createtodo(State(state):State<AppState>,Form(data):Form<CreateTodo>) -> axum::response::Html<String>{
    state.database.execute("INSERT INTO TODO (id,todo,completed,priority,member) VALUES (1,'hai',false,'high','vaasu')");
    println!("{:?}",data);
    format!(
        r#"
        <!doctype html>
        <html>
        <head>
        <title>Book</title>
        </head>
        <body>
        <h1>Book</h1>
        </body>
        </html>
        "#,
        ).into()
}

#[derive(Clone)]
pub struct AppState{
   pub database:Arc<Client>
}

#[tokio::main]
async fn main() {
    const DB_PATH : &str = "file:///home/shastri/deez/todo_mvc/todo.db";
    let config = Config::new(DB_PATH).unwrap();

    let db = Arc::new(libsql_client::Client::from_config(config).await.unwrap());
    db.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, todo TEXT, completed BOOLEAN, priority TEXT, member TEXT)").await.unwrap();    

    let app = Router::new()
        .with_state(db.clone())
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index))
        .route("/todo/add", get(addtodo).post(createtodo));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

