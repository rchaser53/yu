use reqwest;
use reqwest::Client;
use std::env;

mod hotel_info;
use hotel_info::HotelInfoResponse;
mod vacant_info;
use vacant_info::VacantInfo;

mod url_builder;
use url_builder::URLBuilder;

static HOTEL_SEARCH_URL: &'static str =
    "https://app.rakuten.co.jp/services/api/Travel/SimpleHotelSearch/20170426";
static VACANT_SEARCH_URL: &'static str =
    "https://app.rakuten.co.jp/services/api/Travel/VacantHotelSearch/20170426";

fn main() {
    // let mut hotel_search_condition = create_serach_condition(vec![
    //     ("middleClassCode", "akita"),
    //     ("smallClassCode", "tazawa"),
    // ]);
    // let task = get_hotel_info(&mut hotel_search_condition);
    // let _ = futures::executor::block_on(task);

    let mut hotel_search_condition = create_serach_condition(vec![
        ("middleClassCode", "akita"),
        ("smallClassCode", "tazawa"),
        ("checkinDate", "2019-12-01"),
        ("checkoutDate", "2019-12-02"),
        ("adultNum", "1"),
    ]);
    let task = get_vacant_info(&mut hotel_search_condition);
    let _ = futures::executor::block_on(task);
}

fn create_serach_condition(input: Vec<(&str, &str)>) -> Vec<(String, String)> {
    let application_key = (env::var("APPLICATION_KEY"))
        .expect("should export environment variable APPLICATION_KEY to use RAKUTEN api");

    let mut result: Vec<(String, String)> = input
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
    result.push((String::from("largeClassCode"), String::from("japan")));
    result.push((String::from("format"), String::from("json")));
    result.push((String::from("applicationId"), application_key));
    result
}

pub async fn get_hotel_info(
    earch_condition: &mut Vec<(String, String)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut url_builder = URLBuilder::new(HOTEL_SEARCH_URL.to_string());
    url_builder.add_queries(earch_condition);

    let endpoint_url = url_builder.to_string();
    let builder = Client::builder().build()?;
    let body = builder.get(&endpoint_url).send()?.text()?;

    let data: HotelInfoResponse = match serde_json::from_str(&body) {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    for hotel in data.hotels {
        println!("{}", hotel);
    }
    Ok(())
}

pub async fn get_vacant_info(
    earch_condition: &mut Vec<(String, String)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut url_builder = URLBuilder::new(VACANT_SEARCH_URL.to_string());
    url_builder.add_queries(earch_condition);

    let endpoint_url = url_builder.to_string();
    let builder = Client::builder().build()?;
    let body = builder.get(&endpoint_url).send()?.text()?;

    let data: VacantInfo = match serde_json::from_str(&body) {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    println!("{:?}", data);
    Ok(())
}
