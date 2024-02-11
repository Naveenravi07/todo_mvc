use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub todos : Vec<CreateTodo>
}


#[derive(Template)]
#[template(path = "addTodo.html")]
pub struct Addtodo {
    pub default_value: Option<CreateTodo>
}

#[derive(Debug,Deserialize,)]
pub struct CreateTodo {
 pub   id : u32,
 pub   todo : String,
 pub   priority : String,
 pub   member : String
}
#[derive(Debug,Deserialize,)]
pub struct DeleteTodo {
 pub   id : u32,
}
