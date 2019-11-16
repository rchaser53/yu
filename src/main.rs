use reqwest;
use reqwest::Client;
use std::env;
// use futures::Future;
// use reqwest::{Response, Error};

fn main() {
    let task = foo();
    let _ = futures::executor::block_on(task);
}

async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    let application_key = (env::var("APPLICATION_KEY"))
        .expect("should export environment variable APPLICATION_KEY to use RAKUTEN api");

    let endpoint_url = format!("https://app.rakuten.co.jp/services/api/Travel/SimpleHotelSearch/20170426?format=json&largeClassCode=japan&middleClassCode=akita&smallClassCode=tazawa&applicationId={}", application_key);
    let builder = Client::builder().build()?;
    let body = builder.get(&endpoint_url).send()?.text()?;

    println!("body = {:?}", body);
    Ok(())
}
