use rocket::{get, routes};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rocket::State;

struct AddressMap {
    data: HashMap<String, String>,
}

impl AddressMap {
    fn new() -> AddressMap {
        AddressMap {
            data: HashMap::new(),
        }
    }

    fn load_map(&mut self) {
        if let Ok(file) = File::open("map.txt") {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() == 2 {
                        self.data.insert(parts[0].to_string(), parts[1].to_string());
                    }
                }
            }
        }
    }

    fn get_data(&self, address: &str) -> Option<&String> {
        self.data.get(address)
    }
}

#[get("/data?<address>")]
fn get_data(address: String, map: &State<AddressMap>) -> Option<String> {
    map.get_data(&address).cloned()
}

#[rocket::launch]
fn rocket() -> _ {
    let mut map = AddressMap::new();
    map.load_map();

    rocket::build()
        .manage(map)
        .mount("/", routes![get_data])
}