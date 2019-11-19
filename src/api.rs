use reqwest::Client;

use crate::url_builder::URLBuilder;
use crate::vacant_info::{Hotel, RoomInfo, VacantInfo};

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

pub async fn get_vacant_info(
    url_builder: URLBuilder,
) -> Result<String, Box<dyn std::error::Error>> {
    let endpoint_url = url_builder.to_string();
    let builder = Client::builder().build()?;
    let body = builder.get(&endpoint_url).send()?.text()?;

    let data: VacantInfo = match serde_json::from_str(&body) {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    let mut result = String::from("");
    if let Some(first) = data.hotels.first() {
        if let Some(Hotel::RoomInfo(last_vec)) = first.hotel.last() {
            if let Some(RoomInfo::DailyCharge(ref last)) = last_vec.last() {
                if let Some(stay_date) = &last.stay_date {
                    result.push_str(&format!("stay-date: {}\n", stay_date));
                }
            }
        }
    }

    for hotel in data.hotels {
        result.push_str(&format!("{}\n", hotel));
    }

    Ok(result)
}
