use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelInfoResponse {
    pub paging_info: PagingInfo,
    pub hotels: Vec<Hotels>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PagingInfo {
    pub record_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hotels {
    pub hotel: Vec<Hotel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hotel {
    pub hotel_basic_info: Option<HotelBasicInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HotelBasicInfo {
    pub hotel_no: usize,
    pub hotel_name: String,
    pub hotel_information_url: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub access: Option<String>,
    pub nearest_station: Option<String>,
    pub review_count: Option<usize>,
    pub review_average: Option<f32>,
}

impl fmt::Display for Hotels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_vec = Vec::with_capacity(self.hotel.len());
        let result = self
            .hotel
            .iter()
            .fold(&mut string_vec, |stack, hotel| {
                if let Some(ref hotel) = hotel.hotel_basic_info {
                    stack.push(format!("{}", hotel));
                }
                stack
            })
            .join("\n");

        write!(f, "{}", result)
    }
}

macro_rules! output_field {
    ($info:expr) => {
        if let Some(val) = $info {
            format!("{}", val)
        } else {
            String::from("")
        }
    };
}

impl fmt::Display for HotelBasicInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("");
        result.push_str(&format!("hotel_no               {}\n", self.hotel_no));
        result.push_str(&format!("hotel_name             {}\n", self.hotel_name));
        result.push_str(&format!(
            "hotel_information_url, {}\n",
            output_field!(&self.hotel_information_url)
        ));
        result.push_str(&format!(
            "address1,              {}\n",
            output_field!(&self.address1)
        ));
        result.push_str(&format!(
            "address2,              {}\n",
            output_field!(&self.address2)
        ));
        result.push_str(&format!(
            "access,                {}\n",
            output_field!(&self.access)
        ));
        result.push_str(&format!(
            "nearest_station,       {}\n",
            output_field!(&self.nearest_station)
        ));
        result.push_str(&format!(
            "review_count,          {}\n",
            output_field!(self.review_count)
        ));
        result.push_str(&format!(
            "review_average,        {}\n",
            output_field!(self.review_average)
        ));
        write!(f, "{}", result)
    }
}
