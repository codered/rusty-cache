use warp::Filter;
use std::collections::HashMap;
use std::str;

#[tokio::main]
async fn main() {

    let mut cache: HashMap<String, String> = HashMap::new();
    // cache.insert("a".to_string(),"hello".to_string());
    let mut my_closure = || {
                cache.insert("a".to_string(),"1".to_string());
            };

    my_closure();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let set_value = warp::path!("set" / String / String)
        .map(|k: String,v: String| {
            // cache[k] = v;    
            move || cache.insert("a".to_string(),"1".to_string());                    
            format!("{},{}", k,v)
        });



    warp::serve(hello.or(set_value))
        .run(([127, 0, 0, 1], 3030))
        .await;
}