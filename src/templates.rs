use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}


#[derive(Template)]
#[template(path = "addTodo.html")]
pub struct Addtodo {}

