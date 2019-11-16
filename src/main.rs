use reqwest;
use reqwest::Client;
use std::env;

mod url_builder;
use url_builder::URLBuilder;

// use futures::Future;
// use reqwest::{Response, Error};

static HOTEL_SEARCH_URL: &'static str =
    "https://app.rakuten.co.jp/services/api/Travel/SimpleHotelSearch/20170426";
static VACANT_SEARCH_URL: &'static str =
    "https://app.rakuten.co.jp/services/api/Travel/VacantHotelSearch/20170426";

fn main() {
    let task = foo();
    let _ = futures::executor::block_on(task);
}

async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    let application_key = (env::var("APPLICATION_KEY"))
        .expect("should export environment variable APPLICATION_KEY to use RAKUTEN api");

    let mut url_builder = URLBuilder::new(HOTEL_SEARCH_URL.to_string());
    url_builder.add_queries(&mut vec![
        ("format", "json"),
        ("largeClassCode", "japan"),
        ("middleClassCode", "akita"),
        ("smallClassCode", "tazawa"),
        ("applicationId", &application_key),
    ]);

    let endpoint_url = url_builder.to_string();
    let builder = Client::builder().build()?;
    let body = builder.get(&endpoint_url).send()?.text()?;

    println!("body = {:?}", body);
    Ok(())
}
