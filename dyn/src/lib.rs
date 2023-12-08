use serde_json::{Value,json};

#[no_mangle]
pub fn page_function (a: i32, b: i32) -> Value { 
    println!("inside pageFunction");
    let output = json!({ "result": a + b});
    println!("inside pageFunction output: {:?}", output);
    output
}