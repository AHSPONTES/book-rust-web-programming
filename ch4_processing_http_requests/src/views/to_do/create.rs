use actix_web::HttpRequest;
use serde_json::{value::Value, Map};

use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do;

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./state.json");

    let title = req.match_info().get("title").unwrap();

    let item = to_do::to_do_factory(&String::from("pending"), title).expect("create");

    process_input(item, "create".to_string(), &state);

    format!("{} created", title)
}
