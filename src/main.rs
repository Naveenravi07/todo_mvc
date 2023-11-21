mod templates;
use libsql_client::Config;
use tower_http::services::ServeDir;
use axum::{ routing::get, Router, Json};
use templates::{Index, Addtodo};

async fn index() -> Index {
    return Index{} 
}

async fn addtodo() -> Addtodo {
    return Addtodo{} 
}

async fn createtodo(form:Json<serde_json::Value>) -> axum::response::Html<String>{
    println!("{:?}",form);
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

#[tokio::main]
async fn main() {
    const DB_PATH : &str = "file:///home/shastri/deez/todo_mvc/todo.db";
    let config = Config::new(DB_PATH).unwrap();

    let db = libsql_client::Client::from_config(config).await.unwrap();
    db.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, todo TEXT, completed BOOLEAN, priority TEXT, member TEXT)").await.unwrap();    


    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index))
        .route("/todo/add", get(addtodo).post(createtodo));
        
    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

