use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use warp::{http, Filter};

type Items = HashMap<String, String>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Id {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    value: String,
}

#[derive(Clone)]
struct Store {
    cache_list: Arc<RwLock<Items>>,
}

impl Store {
    fn new() -> Self {
        Store {
            cache_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn update_cache_list(item: Item, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.cache_list.write().insert(item.name, item.value);

    Ok(warp::reply::with_status(
        "Added items to the cache",
        http::StatusCode::CREATED,
    ))
}

async fn delete_cache_list_item(id: Id, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.cache_list.write().remove(&id.name);

    Ok(warp::reply::with_status(
        "Removed item from cache list",
        http::StatusCode::OK,
    ))
}

async fn clear_cache_list(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.cache_list.write().clear();
    // this shrinks the hashmap back to 0, this maybe good to free up memory but it may also impact
    // performance at larger scale with lots of insertions.
    store.cache_list.write().shrink_to(0);

    Ok(warp::reply::with_status(
        "Cache has been cleared",
        http::StatusCode::OK,
    ))
}

async fn get_cache_list(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.cache_list.read();
    Ok(warp::reply::json(&*r))
}

async fn get_cache_item(id: Id, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.cache_list.read().clone();
    Ok(warp::reply::json(&r.get(&id.name)))
}

fn get_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let get_item = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("cache"))
        .and(warp::path("item"))
        .and(warp::path::end())
        .and(get_json())
        .and(store_filter.clone())
        .and_then(get_cache_item);

    let clear_cache = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("cache"))
        .and(warp::path("clear"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(clear_cache_list);

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("cache"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_cache_list);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("cache"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_cache_list);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("cache"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_cache_list_item);

    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("cache"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_cache_list);

    let routes = add_items
        .or(get_items)
        .or(delete_item)
        .or(update_item)
        .or(clear_cache)
        .or(get_item);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
