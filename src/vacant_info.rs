use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! output_field {
    ($info:expr) => {
        if let Some(val) = $info {
            format!("{}", val)
        } else {
            String::from("")
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VacantInfo {
    pub paging_info: PagingInfo,
    pub hotels: Vec<Hotels>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PagingInfo {
    pub record_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hotels {
    pub hotel: Vec<Hotel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Hotel {
    RoomInfo(Vec<RoomInfo>),
    HotelBasicInfo(HotelBasicInfo),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RoomInfo {
    RoomBasicInfo(RoomBasicInfo),
    DailyCharge(DailyCharge),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoomBasicInfo {
    pub plan_id: Option<usize>,
    pub plan_name: Option<String>,
    pub with_dinner_flag: usize,
    pub dinner_select_flag: usize,
    pub with_breakfast_flag: usize,
    pub breakfast_select_flag: usize,
    pub salesform_flag: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DailyCharge {
    pub stay_date: Option<String>,
    pub rakuten_charge: usize,
    pub total: usize,
    pub charge_flag: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HotelBasicInfo {
    pub hotel_no: usize,
    pub hotel_name: String,
    pub access: Option<String>,
    pub nearest_station: Option<String>,
    pub review_average: Option<f32>,
}

impl fmt::Display for Hotels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_vec = Vec::with_capacity(self.hotel.len());
        let mut should_break = false;
        let result = self
            .hotel
            .iter()
            .fold(&mut string_vec, |stack, hotel| {
                if should_break {
                    return stack;
                }
                match hotel {
                    Hotel::RoomInfo(room_info_vec) => {
                        let mut room_info_strs = String::from("");
                        for room_info in room_info_vec {
                            room_info_strs.push_str(&format!("{}", room_info));
                        }
                        stack.push(room_info_strs);
                    }
                    Hotel::HotelBasicInfo(hotel_info) => {
                        if let Some(average) = hotel_info.review_average {
                            if average < 3.8 {
                                should_break = true;
                                return stack;
                            }

                            stack.push(format!(
                                "***************************************
name:             {}
access:           {}
near station:     {}
review:           {}
=======================================",
                                &hotel_info.hotel_name,
                                output_field!(&hotel_info.nearest_station),
                                output_field!(&hotel_info.access),
                                average,
                            ));
                        } else {
                            should_break = true;
                            return stack;
                        }
                    }
                };
                stack
            })
            .join("\n");

        write!(f, "{}", result)
    }
}

impl fmt::Display for RoomInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RoomInfo::RoomBasicInfo(basic_info) => write!(f, "{}", basic_info),
            RoomInfo::DailyCharge(daily_charge) => write!(f, "{}", daily_charge),
        }
    }
}

impl fmt::Display for RoomBasicInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("");
        result.push_str(&format!(
            "plan_id:                {}\n",
            output_field!(&self.plan_id)
        ));
        result.push_str(&format!(
            "plan_name:              {}\n",
            output_field!(&self.plan_name)
        ));
        result.push_str(&format!(
            "with_breakfast_flag:    {}\n",
            self.with_breakfast_flag
        ));
        write!(f, "{}", result)
    }
}

impl fmt::Display for DailyCharge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("");
        result.push_str(&format!("total:                  {}\n", self.total));
        write!(f, "{}", result)
    }
}
