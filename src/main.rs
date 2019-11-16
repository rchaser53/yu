use reqwest;
use reqwest::{Client, ClientBuilder};
// use futures::Future;
// use reqwest::{Response, Error};

fn main() {
    let task = foo();
    let _ = futures::executor::block_on(task);
}

async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    let builder = Client::builder().build()?;
    let body = builder.get("https://www.rust-lang.org").send()?.text()?;

    println!("body = {:?}", body);
    Ok(())
}
