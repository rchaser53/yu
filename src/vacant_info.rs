use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VacantInfo {
    pub paging_info: PagingInfo,
    pub hotels: Option<Vec<EnumField>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PagingInfo {
    pub record_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumField {
    pub hotel: Vec<Hotel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Hotel {
    RoomInfo(Vec<RoomInfo>),
    HotelBasicInfo(Option<HotelBasicInfo>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RoomInfo {
    RoomBasicInfo(RoomDetailInfo),
    DailyCharge(DailyCharge),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoomDetailInfo {
    // pub plan_id: Option<usize>,
    pub plan_name: Option<String>,
    // pub with_dinner_flag: usize,
    // pub dinner_select_flag: usize,
    // pub with_breakfast_flag: usize,
    // pub breakfast_select_flag: usize,
    // pub salesform_flag: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DailyCharge {
    // pub plan_id: Option<usize>,
    pub stay_date: Option<String>,
    // pub with_dinner_flag: usize,
    // pub dinner_select_flag: usize,
    // pub with_breakfast_flag: usize,
    // pub breakfast_select_flag: usize,
    // pub salesform_flag: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HotelBasicInfo {
    pub hotel_no: usize,
}
