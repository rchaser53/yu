#[macro_use]
extern crate anyhow;

use anyhow::Result;

mod hotel_info;
mod vacant_info;

mod config;
use config::create_url_builder;
mod api;
use api::get_vacant_info;

mod url_builder;

fn main() -> Result<()> {
    let url_builders = create_url_builder("condition.toml")?;
    for url_builder in url_builders {
        let task = get_vacant_info(url_builder);
        match futures::executor::block_on(task) {
            Ok(result) => println!("{}", result),
            Err(err) => panic!("{}", err),
        };
    }
    Ok(())
}
