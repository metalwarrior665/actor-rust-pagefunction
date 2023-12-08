#[macro_use] extern crate serde_derive;

mod apify;
mod page_function_example;

use std::process::Command;
use crate::apify::{get_value,push_data};
use serde_json::Value;
use reqwest::Client;
use scraper::{Html};

#[tokio::main]
async fn main() {
    let input = get_value("INPUT").await.unwrap();
    println!("loaded input: {}", input.page_function);

    std::fs::write("./dyn/src/lib.rs", input.page_function).unwrap();
    println!("wrote page_function to ./dyn/src/lib.rs");

    // cargo build --manifest-path=./dyn/Cargo.toml
    let mut build_command = Command::new("cargo");

    build_command.arg("build");

    if input.build_type == "release" {
        build_command.arg("--release");
    }
    let output = 
        build_command
        .arg("--manifest-path=./dyn/Cargo.toml")
        .output()
        .expect("failed to execute process");

    println!("compiled into dynamic library output: {:?}", String::from_utf8(output.stdout).unwrap());
    println!("compiled into dynamic library error: {:?}", String::from_utf8(output.stderr).unwrap());

    fn call_dynamic(document: Html, build_type: String) -> Result<Value, Box<dyn std::error::Error>> {
        let library_path = match std::env::consts::OS {
            "macos" => format!("dyn/target/{}/liblibrary.dylib", build_type),
            "linux" => format!("dyn/target/{}/liblibrary.so", build_type),
            _ => panic!("Unsupported OS"),
        };
        println!("loading dynamic library from path: {}", library_path);
        unsafe {
            let lib = libloading::Library::new(library_path)?;
    
            let func: libloading::Symbol<unsafe extern fn(Html) -> Value> = lib.get(b"page_function")?;
            Ok(func(document))
        }
    }

    let client = Client::new();

    let response = client.get(input.url).send().await.unwrap();
    let html = response.text().await.unwrap();
    let document = Html::parse_document(&html);

    let page_function_output = call_dynamic(document, input.build_type).unwrap();
    println!("page_function finished with result: {:?}", page_function_output );

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
