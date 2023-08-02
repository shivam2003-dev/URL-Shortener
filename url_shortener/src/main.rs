use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Filter;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use warp::http::Uri;
use std::str::FromStr;
use warp::reply::json;

type Urls = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    let urls = Arc::new(Mutex::new(HashMap::new()));

    let shorten = warp::path!("shorten")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_urls(urls.clone()))
        .and_then(shorten_url);

    let redirect = warp::path!(String)
        .and(with_urls(urls.clone()))
        .and_then(redirect_url);

    let routes = shorten.or(redirect);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_urls(urls: Urls) -> impl Filter<Extract = (Urls,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || urls.clone())
}

async fn shorten_url(body: HashMap<String, String>, urls: Urls) -> Result<impl warp::Reply, warp::Rejection> {
    let mut urls = urls.lock().unwrap();
    let original_url = body.get("url").unwrap().clone();
    let mut hasher = DefaultHasher::new();
    original_url.hash(&mut hasher);
    let short_code = format!("{:x}", hasher.finish());
    urls.insert(short_code.clone(), original_url);
    Ok(json(&short_code))
}

async fn redirect_url(short_code: String, urls: Urls) -> Result<impl warp::Reply, warp::Rejection> {
    let urls = urls.lock().unwrap();
    match urls.get(&short_code) {
        Some(url) => Ok(warp::redirect::see_other(Uri::from_str(url).unwrap())),
        None => Err(warp::reject::not_found()),
    }
}

