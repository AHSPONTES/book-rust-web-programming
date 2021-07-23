#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_service::Service;
use actix_web::{App, HttpServer};

mod database;
mod json_serialization;
mod models;
mod processes;
mod schema;
mod state;
mod to_do;
mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                if *&req.path().contains("/item/") {
                    match views::token::process_token(&req) {
                        Ok(_token) => println!("the token is passable"),
                        Err(message) => println!("token error: {}", message),
                    }
                }
                let fut = srv.call(req);
                async {
                    let result = fut.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

/*
let args: Vec<String> = env::args().collect();

let command = &args[1];
let title = &args[2];

let file_name = "./state.json";
let mut state: Map<String, Value> = read_file(file_name);

let status = match &state.get(*&title) {
    Some(result) => result.to_string().replace('\"', ""),
    None => String::from("pending"),
};

let item = to_do_factory(&status, title).expect(&status);

process_input(item, command.to_string(), &state);
*/
