#[macro_use] extern crate serde_derive;

mod apify;

use std::process::Command;
use crate::apify::{get_value,push_data};
use serde_json::Value;
use reqwest::Client;
#[tokio::main]
async fn main() {
    let value = get_value("INPUT").await.unwrap();
    println!("page fn: {}", value.page_function);

    std::fs::write("./dyn/src/lib.rs", value.page_function).unwrap();

    // cargo build --manifest-path=./dyn/Cargo.toml
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path=./dyn/Cargo.toml")
        .output()
        .expect("failed to execute process");

    println!("output: {:?}", String::from_utf8(output.stdout).unwrap());
    println!("error: {:?}", String::from_utf8(output.stderr).unwrap());


    fn call_dynamic() -> Result<Value, Box<dyn std::error::Error>> {
        unsafe {
            let lib = match libloading::Library::new("dyn/target/release/liblibrary.dylib") {
                Ok(lib) => lib,
                Err(e) => {
                    println!("Cannot find dyn/target/release/liblibrary.dylib, will try .so");
                    libloading::Library::new("dyn/target/release/liblibrary.so").unwrap()
                },
            };
            let func: libloading::Symbol<unsafe extern fn(i32, i32) -> Value> = lib.get(b"page_function")?;
            Ok(func(1, 2))
        }
    }

    let client = Client::new();

    let page_function_output = call_dynamic().unwrap();
    println!("call_dynamic: {:?}", page_function_output );

    // wrap to array if it's not already
    let to_push = match page_function_output {
        Value::Array(array) => array,
        _ => vec![page_function_output],
    };
    match push_data(to_push, &client, false).await {
        Ok(_) => {
            println!("Data pushed successfully");
        },
        Err(e) => {
            println!("Error pushing data: {}", e);
        }
    }; 
}
