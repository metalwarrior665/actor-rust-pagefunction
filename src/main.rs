#[macro_use] extern crate serde_derive;

mod apify;

use std::process::Command;
use crate::apify::{get_value,push_data};
#[tokio::main]
async fn main() {
    let value = get_value("INPUT").await.unwrap();
    println!("page fn: {}", value.pageFunction);

    std::fs::write("./dyn/src/lib.rs", value.pageFunction).unwrap();

    // cargo build --manifest-path=./dyn/Cargo.toml
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path=./dyn/Cargo.toml")
        .output()
        .expect("failed to execute process");

    println!("output: {:?}", String::from_utf8(output.stdout).unwrap());
    println!("error: {:?}", String::from_utf8(output.stderr).unwrap());


    fn call_dynamic() -> Result<u32, Box<dyn std::error::Error>> {
        unsafe {
            let lib = match libloading::Library::new("dyn/target/release/liblibrary.dylib") {
                Ok(lib) => lib,
                Err(e) => {
                    println!("Cannot find dyn/target/release/liblibrary.dylib, will try .so");
                    libloading::Library::new("dyn/target/release/liblibrary.so").unwrap()
                },
            };
            let func: libloading::Symbol<unsafe extern fn(i32, i32) -> u32> = lib.get(b"sum")?;
            Ok(func(1, 2))
        }
    }

    println!("call_dynamic: {:?}", call_dynamic());
}
