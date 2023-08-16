use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::{self, Filter};
use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() {
    // Populate this map with your data
    let data_map: HashMap<String, String> = [
        ("0xfdfadfafd".to_string(), "12323".to_string()),
        ("0xfdfadfafe".to_string(), "1953".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

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