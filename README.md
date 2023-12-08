Example actor showcasing running a user-provided function in a static-typed compiled language.

## How does it work
1. Reads the input from disk or via Apify API
2. Extracts the `page_function` string from the input
3. Stores the `page_function` string to the disk
4. Spawns a system process using `cargo` to compile the `page_function` into a C dynamic library (Rust doesn't have stable ABI so C ABI must be used)
5. Dynamically links the library and converts the `page_function` into a regular Rust function. It must adhere to predefined input/output types. 
6. The example code gets HTML from the input provided `url` and parses it into a `document` using the [Scraper](https://docs.rs/scraper/latest/scraper/) library 
7. The user-provided `page_function` gets the `document` as an input parameter and returns a JSON [Value](https://docs.rs/serde_json/latest/serde_json/enum.Value.html) type using the `json` macro

## Page function
Page function can use a predefined set of Rust libraries, currently only the [Scraper](https://docs.rs/scraper/latest/scraper/) library and [serde_json](https://docs.rs/serde_json/latest/serde_json/) for JSON `Value` type are provided. 

### TODO
But technically, thanks to dynamic compiling, we can enable users to provide a list of libraries to be used in the `page_function`.

### Example page_function
```rust
use serde_json::{Value,json};
use scraper::{Html, Selector};

fn selector_to_text(document: &Html, selector: &str) -> Option<String> {
    document
        .select(&Selector::parse(selector).unwrap())
        .next()
        .map(|el| el.text().next().unwrap().into() )
}
#[no_mangle]
pub fn page_function (document: Html) -> Value { 
    println!("inside pageFunction");

    let title = selector_to_text(&document, "title");
    let header = selector_to_text(&document, "h1");

    let companies_using_apify = document
        .select(&Selector::parse(".Logos__container").unwrap())
        .next().unwrap()
        .select(&Selector::parse("img").unwrap())
        .map(|el| el.value().attr("alt").unwrap().to_string())
        .collect::<Vec<String>>();
    let output = json!({
        "title": title,
        "header": header,
        "companies_using_apify": companies_using_apify,
    });
    println!("inside pageFunction output: {:?}", output);
    output
}
```