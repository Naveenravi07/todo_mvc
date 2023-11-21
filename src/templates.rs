use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}


#[derive(Template)]
#[template(path = "addTodo.html")]
pub struct Addtodo {}

#[derive(Debug)]
pub struct CreateTodo {
    id : u32,
    todo : String,
    priority : String,
    member : String
}
