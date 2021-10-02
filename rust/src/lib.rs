#[macro_use] extern crate serde_derive;

mod atcf;

mod jni;

pub fn say_hello() -> String {
    "Hello from safe Rust!".to_owned()
}

