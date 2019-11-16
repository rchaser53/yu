use reqwest;
// use futures::Future;
// use reqwest::{Response, Error};

fn main() {
    let task = foo();
    let _ = futures::executor::block_on(task);
}

async fn foo() -> Result<(), Box<dyn std::error::Error>>{
  let body = reqwest::get("https://www.rust-lang.org")?
    .text()?;

  println!("body = {:?}", body);
  Ok(())
}