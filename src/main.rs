mod templates;
use libsql_client::Config;
use tower_http::{services::ServeDir, cors::CorsLayer};
use axum::{ routing::{get, post}, Router, Json, http::{HeaderValue, Method, header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE}}};
use templates::{Index, Addtodo, CreateTodo};

async fn index() -> Index {
    return Index{} 
}

async fn addtodo() -> Addtodo {
    return Addtodo{} 
}

async fn createtodo(Json(form):Json<serde_json::Value>) -> axum::response::Html<String>{
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
    const DB_PATH : &str = "file:///home/shastri/balls/todo_mvc/todo.db";
    let config = Config::new(DB_PATH).unwrap();

    let db = libsql_client::Client::from_config(config).await.unwrap();
    db.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, todo TEXT, completed BOOLEAN, priority TEXT, member TEXT)").await.unwrap();    



    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index))
        .route("/todo/add", get(addtodo).post(createtodo));
        
    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.layer(cors).into_make_service())
        .await
        .unwrap();
}

