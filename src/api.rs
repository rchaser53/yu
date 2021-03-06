use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct RakutenAPIError {
    error: String,
    error_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Res {
    Hotels(VacantInfo),
    Error(RakutenAPIError),
}

pub async fn get_vacant_info(url_builder: URLBuilder) -> Result<String> {
    let endpoint_url = url_builder.to_string();
    let builder = Client::builder().build()?;
    let body = builder.get(&endpoint_url).send()?.text()?;

    let hotels = match serde_json::from_str(&body) {
        Ok(Res::Hotels(VacantInfo { hotels, .. })) => hotels,
        Ok(Res::Error(RakutenAPIError {
            error_description,
            error,
        })) => {
            if error == "not_found" {
                return Err(anyhow!("no hotels hit"));
            }
            panic!("{}", error_description)
        }
        Err(err) => panic!("{}", err),
    };

    let mut result = String::from("");
    if let Some(first) = hotels.first() {
        if let Some(Hotel::RoomInfo(last_vec)) = first.hotel.last() {
            if let Some(RoomInfo::DailyCharge(ref last)) = last_vec.last() {
                if let Some(stay_date) = &last.stay_date {
                    result.push_str(&format!("stay-date: {}\n", stay_date));
                }
            }
        }
    }

    for hotel in hotels {
        result.push_str(&format!("{}\n", hotel));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::url_builder::URLBuilder;
    use futures::executor::block_on;
    use mockito::{mock, server_url};
    use serde_json::json;

    #[test]
    fn use_mockito_test() {
        let res = json!({
          "pagingInfo": {
            "recordCount": 0,
          },
          "hotels": []
        });

        let _m = mock("GET", "/?")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(res.to_string())
            .create();
        let url = server_url();
        let url_builder = URLBuilder::new(url);
        let result = block_on(get_vacant_info(url_builder)).unwrap();
        assert_eq!(result, "");
    }
}
