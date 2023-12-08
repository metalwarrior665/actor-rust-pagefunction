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
    println!("page_function starting");

    let title = selector_to_text(&document, "title");
    println!("extracted title: {:?}", title);

    let header = selector_to_text(&document, "h1");
    println!("extracted header: {:?}", header);

    let companies_using_apify = document
        .select(&Selector::parse(".Logos__container").unwrap())
        .next().unwrap()
        .select(&Selector::parse("img").unwrap())
        .map(|el| el.value().attr("alt").unwrap().to_string())
        .collect::<Vec<String>>();

    println!("extracted companies_using_apify: {:?}", companies_using_apify);

    let output = json!({
        "title": title,
        "header": header,
        "companies_using_apify": companies_using_apify,
    });
    println!("inside pageFunction output: {:?}", output);
    output
}