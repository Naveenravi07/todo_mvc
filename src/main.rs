mod templates;
use axum::{
    debug_handler,
    extract::State,
    routing::{delete, get},
    Form, Router,
};
use libsql_client::{args, de, Client, Config, Statement};
use std::sync::Arc;
use templates::{Addtodo, CreateTodo, DeleteTodo, Index};
use tower_http::services::ServeDir;

async fn index(State(state): State<AppState>) -> Index {
    let todos = state.database.execute("SELECT * FROM todos").await.unwrap();

    let todos = todos
        .rows
        .iter()
        .map(|item| de::from_row(item))
        .collect::<Result<Vec<CreateTodo>, _>>()
        .unwrap();
    print!("{:?}", todos);
    Index { todos }
}

async fn addtodo() -> Addtodo {
    Addtodo {}
}

async fn delete_todo(
    State(state): State<AppState>,
    Form(data): Form<DeleteTodo>,
) -> axum::response::Html<String> {
    state
        .database
        .execute(Statement::with_args(
            "DELETE FROM todos WHERE id=?",
            args!(data.id),
        ))
        .await
        .unwrap();

    axum::response::Html(format!(""))
}

#[debug_handler]
async fn createtodo(
    State(state): State<AppState>,
    Form(data): Form<CreateTodo>,
) -> axum::response::Html<String> {
    state
        .database
        .execute(Statement::with_args(
            "INSERT INTO todos (id,todo,member,priority,completed) VALUES (?,?,?,?,false)",
            args!(data.id, &data.todo, &data.member, &data.priority),
        ))
        .await
        .unwrap();

    axum::response::Html(format!(
        r#"
            <tr  class="fw-normal">
            <th>
            <img src="https://mdbcdn.b-cdn.net/img/Photos/Avatars/avatar-3.webp"
            class="shadow-1-strong rounded-circle" alt="avatar 1"
            style="width: 55px; height: auto;">
            <span class="ms-2">{member}</span>
            </th>
            <td class="align-middle">{todo}</td>
            <td class="align-middle">
            <h6 class="mb-0"><span class="badge bg-danger">{priority}priority</span></h6>
            </td>
            <td class="align-middle">
            <a  data-mdb-toggle="tooltip" title="Done"><i
            class="fas fa-check text-success me-3"></i></a>
            <a data-mdb-toggle="tooltip" title="Remove"><i
            class="fas fa-trash-alt text-danger"></i></a>
            </td>
            </tr>
            "#,
        member = data.member,
        todo = data.todo,
        priority = data.priority
    ))
}

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<Client>,
}

#[tokio::main]
async fn main() {
    const DB_PATH: &str = "file:///home/shastri/deez/todo_mvc/todo.db";
    let config = Config::new(DB_PATH).expect("Failed to create database configuration");

    let db = Arc::new(
        libsql_client::Client::from_config(config)
            .await
            .expect("Failed to connect to the database"),
    );
    db.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, todo TEXT, completed BOOLEAN, priority TEXT, member TEXT)").await.expect("Failed to create todos table");

    let app_state = AppState { database: db };

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index))
        .route("/todo/add", get(addtodo).post(createtodo))
        .route("/todo/delete", delete(delete_todo))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:42069".parse().expect("Invalid address"))
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}
