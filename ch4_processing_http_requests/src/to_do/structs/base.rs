use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Base {
    pub title: String,
    pub status: String,
}

impl Base {
    pub fn new(title: &str, status: &str) -> Base {
        Base {
            title: title.to_string(),
            status: status.to_string(),
        }
    }
}

#[cfg(test)]
mod base_tests {

    use super::Base;

    #[test]
    fn new() {
        let title: &str = "test title";
        let expected_title: String = String::from("test title");
        let status: &str = "test status";
        let expected_status: String = String::from("test status");

        let new_base_struct: Base = Base::new(title, status);
        assert_eq!(expected_title, new_base_struct.title);
        assert_eq!(expected_status, new_base_struct.status);
    }
}
