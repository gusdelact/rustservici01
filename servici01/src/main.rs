#[macro_use]
extern crate rocket;
use rustddd::dominio01::*;
use std::collections::HashMap;

struct DummyPersistable {
    dummy_database: HashMap<u32, String>,
}
impl DummyPersistable {
    fn init(&mut self) {
        self.dummy_database = HashMap::new();
        self.dummy_database.insert(1, String::from("hola"));
        self.dummy_database.insert(2, String::from("adios"));
        self.dummy_database.insert(3, String::from("mundo"));
        self.dummy_database.insert(4, String::from("nube"));
    }
}
impl Persistable for DummyPersistable {
    fn save(&self, id: u32, message: String) {
        println!("{:?} {:?}", id, message);
    }

    fn load(&self, id: u32) -> String {
        println!("{:?} ", id);
        let result: String = match self.dummy_database.get(&id) {
            Some(m) => m.to_string(),
            None => String::from("NAN"),
        };
        result
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/ddd")]
fn logic() -> String {
        let mut m01 = Dominio01 {
        id: 1,
        message: String::from(""),
    };
    let mut dummy_repo = DummyPersistable {
        dummy_database: HashMap::new(),
    };
    dummy_repo.init();
    let msg: String = m01.logic(&dummy_repo);
    msg
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello])
        .mount("/", routes![logic])
}
