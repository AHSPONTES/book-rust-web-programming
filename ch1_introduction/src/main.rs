/* struct Human {
    name: String,
    age: u8,
    current_thought: String,
}

impl Human {
    fn new(input_name: &str, input_age: u8) -> Human {
        return Human {
            name: input_name.to_string(),
            age: input_age,
            current_thought: String::from("nothing"),
        };
    }

    fn with_thought(mut self, thought: &str) -> Human {
        self.current_thought = thought.to_string();
        return self;
    }

    fn speak(&self) -> () {
        println!(
            "Hello, my name is {} and I'm {} years old ",
            self.name, self.age
        );
    }
}

fn main() {
    let developer = Human::new("andre", 29);
    developer.speak();
    println!("currently I'm thinking {}", developer.current_thought);

    let new_developer = Human::new("maria", 30).with_thought("I am hungry!");
    new_developer.speak();
    println!("currently I'm thinking {}", new_developer.current_thought);
}
 */
/*
use std::collections::HashMap;

enum AllowedData {
    S(String),
    I(i8),
}

struct CustomMap {
    body: HashMap<String, AllowedData>,
}

impl CustomMap {
    fn new() -> CustomMap {
        CustomMap {
            body: HashMap::new(),
        }
    }
    fn get(&self, key: &str) -> &AllowedData {
        self.body.get(key).unwrap()
    }
    fn insert(&mut self, key: &str, value: AllowedData) -> () {
        self.body.insert(key.to_string(), value);
    }
    fn display(&self, key: &str) -> () {
        match self.get(key) {
            AllowedData::I(value) => println!("I {}", value),
            AllowedData::S(value) => println!("S {}", value),
        }
    }
}
fn main() {
    // defining a new hash map
    let mut map = CustomMap::new();

    // inseting two different types of data
    map.insert("test", AllowedData::I(8));
    map.insert("testing", AllowedData::S("80".to_string()));
    map.insert("test", AllowedData::S("8".to_string()));

    // displaying the data
    map.display("test");
    map.display("testing");
}
 */
/*
trait CanEdit {
    fn edit(&self) {
        println!("user is editing");
    }
}
trait CanCreate {
    fn create(&self) {
        println!("user is creating");
    }
}
trait CanDelete {
    fn delete(&self) {
        println!("user is deleting");
    }
}

struct BaseUser {
    name: String,
    password: String,
}
struct AdminUser {
    super_struct: BaseUser,
}
impl CanEdit for AdminUser {}
impl CanCreate for AdminUser {}
impl CanDelete for AdminUser {}

struct GeneralUser {
    super_struct: BaseUser,
    team: String,
}
impl GeneralUser {
    fn new(name: String, password: String, team: String) -> GeneralUser {
        GeneralUser {
            super_struct: BaseUser { name, password },
            team,
        }
    }
}
impl CanEdit for GeneralUser {}
impl CanCreate for GeneralUser {
    fn create(&self) -> () {
        println!(
            "{} is creating under a {} team",
            self.super_struct.name, self.team
        );
    }
}

fn delete<T: CanDelete>(user: T) -> () {
    user.delete();
}

fn main() {
    let person = AdminUser {
        super_struct: BaseUser {
            name: "andre".to_string(),
            password: "senha".to_string(),
        },
    };

    person.delete();
    person.create();
    person.edit();

    let person2 = GeneralUser::new(
        "maria".to_string(),
        "senha".to_string(),
        "cadastro".to_string(),
    );

    person2.create();
    person2.edit();
}
 */

/* struct Coordinate<T> {
    x: T,
    y: T,
}

fn main() {
    let one = Coordinate { x: 1, y: 10 };
    let two = Coordinate { x: 2, y: 10 };
    let three = Coordinate { x: 3.0, y: 10.0 };
}
 */

macro_rules! capitalize {
    ($a: expr) => {
        let mut v: Vec<char> = $a.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        $a = v.into_iter().collect();
    };
}

fn main() {
    let mut x = String::from("andre");
    capitalize!(x);
    println!("{}", x);
}
