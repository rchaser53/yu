use reqwest::Client;

// use crate::hotel_info::HotelInfoResponse;
use crate::url_builder::URLBuilder;
use crate::vacant_info::VacantInfo;

// static HOTEL_SEARCH_URL: &'static str =
//     "https://app.rakuten.co.jp/services/api/Travel/SimpleHotelSearch/20170426";

// pub async fn get_hotel_info(
//     earch_condition: &mut Vec<(String, String)>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let mut url_builder = URLBuilder::new(HOTEL_SEARCH_URL.to_string());
//     url_builder.add_queries(earch_condition);

//     let endpoint_url = url_builder.to_string();
//     let builder = Client::builder().build()?;
//     let body = builder.get(&endpoint_url).send()?.text()?;

//     let data: HotelInfoResponse = match serde_json::from_str(&body) {
//         Ok(data) => data,
//         Err(err) => panic!("{}", err),
//     };

//     for hotel in data.hotels {
//         println!("{}", hotel);
//     }
//     Ok(())
// }

pub async fn get_vacant_info(url_builder: URLBuilder) -> Result<(), Box<dyn std::error::Error>> {
    let endpoint_url = url_builder.to_string();
    let builder = Client::builder().build()?;
    let body = builder.get(&endpoint_url).send()?.text()?;

    let data: VacantInfo = match serde_json::from_str(&body) {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    for hotel in data.hotels {
        println!("{}", hotel);
    }

    Ok(())
}
