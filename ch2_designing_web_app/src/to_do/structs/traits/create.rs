use serde_json::{json, value::Value, Map};

use crate::state::write_to_file;

pub trait Create {
    fn create(&self, title: &str, status: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_to_file("./state.json", state);
        println!("\n\n{} is being created\n\n", title);
    }
}
