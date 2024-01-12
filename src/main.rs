#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate redis_async;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate validator_derive;
extern crate validator;

#[macro_use]
extern crate log;

mod config;
mod database;
mod errors;
mod handlers;
mod helpers;
mod models;
mod schema;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello world");

    Ok(())
}
