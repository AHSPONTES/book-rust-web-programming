mod state;

use serde_json::{json, value::Value, Map};
use state::{read_file, write_to_file};
use std::env;

mod to_do;

use to_do::structs::traits::create::Create;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];
    let file_name: &str = "./state.json";

    let mut state: Map<String, Value> = read_file(file_name);
    println!("{:?}", state);

    state.insert(title.to_string(), json!(status));
    write_to_file(file_name, &mut state);

    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "washing");

    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => {
            println!(
                "it's a done item with the title: {}",
                item.super_struct.title
            );
        }
    }
}
