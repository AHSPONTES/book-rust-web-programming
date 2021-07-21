use actix_web::{App, HttpServer};

mod json_serialization;
mod processes;
mod state;
mod to_do;
mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
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
