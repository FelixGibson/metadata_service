use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::{self, Filter};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Deserialize)]
struct Address {
    address: String,
}

#[derive(Serialize)]
struct Response {
    is_success: bool,
    data: Option<String>,
    error: Option<String>,
}

fn load_map() -> HashMap<String, String> {
    let mut data: HashMap<String, String> = HashMap::new();
    if let Ok(file) = File::open("map.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1..].join(",");
                    data.insert(key, value);
                }
            }
        }
    }
    return data;
}

#[tokio::main]
async fn main() {
    // Populate this map with your data
    let data_map: HashMap<String, String> = load_map();

    let data_map = Arc::new(Mutex::new(data_map));

    let data_route = warp::path("data")
        .and(warp::get())
        .and(warp::query::<Address>())
        .and(with_data_map(data_map.clone()))
        .and_then(data_handler);

    warp::serve(data_route).run(([127, 0, 0, 1], 3030)).await;
}

fn with_data_map(
    data_map: Arc<Mutex<HashMap<String, String>>>,
) -> impl Filter<Extract = (Arc<Mutex<HashMap<String, String>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || data_map.clone())
}

async fn data_handler(
    address: Address,
    data_map: Arc<Mutex<HashMap<String, String>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let data_map = data_map.lock().unwrap();
    match data_map.get(&address.address) {
        Some(data) => {
            let response = Response {
                is_success: true,
                data: Some(data.clone()),
                error: None,
            };
            Ok(warp::reply::json(&response))
        }
        None => {
            let response = Response {
                is_success: false,
                data: None,
                error: Some("not found".to_string()),
            };
            Ok(warp::reply::json(&response))
        }
    }
}