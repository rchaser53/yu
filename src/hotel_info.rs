#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PagingInfo {
    record_count: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelInfoResponse {
    paging_info: PagingInfo,
    hotels: Vec<HotelInfoMiddle>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotelInfoMiddle {
    hotel: Vec<HotelDataWrapper>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HotelDataWrapper {
    hotel_basic_info: Option<HotelData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HotelData {
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
