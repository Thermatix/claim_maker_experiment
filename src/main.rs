#![feature(decl_macro, proc_macro_hygiene)]
use dotenv::dotenv;
use std::env;

mod prelude;
mod controllers;

fn main() {
    dotenv().ok();
    env::vars();
    prelude::routes::mount();
}
