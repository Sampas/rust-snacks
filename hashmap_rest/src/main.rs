#[macro_use]
extern crate rocket;
// use rocket::{get, launch, routes};
use rocket::form::Form;
use rocket::State;
use std::collections::HashMap;
use std::fmt;
use std::sync::RwLock;

#[derive(Debug)]
struct Resource {
    name: String,
    age: u8,
    aliases: Vec<String>,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name: {}, age: {}, aliases: {:?}",
            self.name, self.age, self.aliases
        )
    }
}

struct ResourceData {
    data: RwLock<HashMap<u32, Resource>>,
}

#[get("/resource/<id>")]
fn res_get(data: &State<ResourceData>, id: u32) -> String {
    if let Some(element) = data.data.read().unwrap().get(&id) {
        println!("{:?}", element);
        format!("{}", element)
    } else {
        String::new()
    }
}

#[derive(FromForm)]
struct EntryForm<'r> {
    name: &'r str,
    #[field(default = 50)]
    age: u8,
}

#[post("/resource", data = "<entry>")]
fn res_post(data: &State<ResourceData>, entry: Form<EntryForm<'_>>) -> String {
    let element = Resource {
        name: entry.name.to_string(),
        age: entry.age,
        aliases: vec![],
    };
    let mut map = data.data.write().unwrap();
    if let Some(max_key) = map.keys().max() {
        let next_key = max_key + 1;
        map.insert(next_key, element);
        format!("http://127.0.0.1:8000/resource/{}", next_key)
    } else {
        map.insert(1, element);
        format!("http://127.0.0.1:8000/resource/1")
    }
}

#[launch]
fn rocket() -> _ {
    let data = ResourceData {
        data: RwLock::new(HashMap::new()),
    };

    data.data.write().unwrap().insert(
        1337,
        Resource {
            name: "John Doe".to_string(),
            age: 21,
            aliases: vec!["Jane".to_string()],
        },
    );

    rocket::build()
        .manage(data)
        .mount("/", routes![res_get, res_post])
}
