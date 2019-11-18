mod hotel_info;
mod vacant_info;

mod config;
use config::create_url_builder;
mod api;
use api::get_vacant_info;

mod url_builder;

fn main() {
    let url_builders = create_url_builder("condition.toml").unwrap();
    for url_builder in url_builders {
        let task = get_vacant_info(url_builder);
        let _ = futures::executor::block_on(task);
    }
}
