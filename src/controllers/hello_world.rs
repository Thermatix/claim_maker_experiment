use rocket_codegen::{get};
// use rocket_codegen::{get, post, patch, put, delete, head, options, catch};

#[get("/hello/<name>/<age>")]
pub fn greet(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
