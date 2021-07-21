use serde_json::{value::Value, Map};

use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::to_do::{to_do_factory, ItemTypes};

pub fn return_state() -> ToDoItems {
    let state: Map<String, Value> = read_file("./state.json");

    let mut array_buffer = Vec::new();

    for (key, value) in state {
        let item_type: &str = value.as_str().unwrap();
        let item: ItemTypes = to_do_factory(item_type, key.as_str()).unwrap();
        array_buffer.push(item);
    }

    ToDoItems::new(array_buffer)
}
