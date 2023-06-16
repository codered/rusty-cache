
#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
use std::sync::Arc;
use rocket::State;
use std::sync::Mutex;
use std::collections::HashMap;

struct MyConfig {
    user_val: HashMap<String, String>
}



// lazy_static! {
//     static ref CACHE: Mutex<HashMap<&'static str, &'static str>> = {
//         let mut m = HashMap::new();
//         Mutex::new(m)
//     };    
// }

// #[derive(FromForm)]
// struct FormData {
//     key: String,
//     value: String,
// }

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<key>")]
fn read(key: &str, state: &State<MyConfig>) -> String {
    format!("KEY: {}. STATE: {:?}", key, state.user_val.get(key))
}

#[get("/create/<key>/<value>")]
fn create(key: &str, value: &str, state: &State<MyConfig> ) -> String { 
    // let mut cloned_hash = state.user_val.clone();
    // cloned_hash.insert(key.to_string(),value.to_string());
    // format!("cloned hash: {:?}", cloned_hash)
    
    let k = key.clone();
    let v = value.clone();
    // Arc::new(state.user_val.insert(&key,&value));
    
    format!("Creating entry {},{}", key,value)
 }

//  #[post("/insert", data = "<data>")]
// fn insert_data(data: Form<FormData>, map: State<HashMap<String, String>>) -> String {
//     let FormData { key, value } = data.into_inner();
//     map.write().insert(key, value);
//     format!("Inserted key: {} with value: {}", key, value)
// }

// #[launch]
// fn rocket() -> _ {    
//     rocket::build()
//     .manage(MyConfig { user_val: HashMap::new()})
//     .mount("/", routes![index, read, create])
   
// }

#[launch]
fn rocket() -> _ {  
    let mut x = Arc::new(HashMap<String, String>);
    *Arc::get_mut(&mut x).unwrap() = 4;
    assert_eq!(*x, 4);
    // let mut map = CACHE.lock().unwrap();
    // map.insert("hello","world");
    rocket::build()
    .manage(MyConfig { user_val: HashMap::new()})
    .mount("/", routes![index, read, create])
}
