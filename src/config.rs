use anyhow::Result;
use chrono::prelude::*;
use serde_derive::Deserialize;
use toml;

use std::cmp::Ordering::Equal;
use std::env;
use std::fs;
use std::path::Path;

use crate::url_builder::URLBuilder;

static VACANT_SEARCH_URL: &'static str =
    "https://app.rakuten.co.jp/services/api/Travel/VacantHotelSearch/20170426";

#[derive(Debug, Deserialize)]
struct SearchConfig {
    pub conditions: Option<Vec<Condition>>,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub squeeze: Vec<String>,
    pub prefecture: String,
    pub area: String,
    pub checkin: String,
    pub checkout: String,
    pub max_charge: usize,
}

fn validate_checkin_date(queries: &mut Vec<(&str, String)>) {
    for elem in queries {
        if elem.0 == "checkinDate" {
            elem.1 = String::from("2020-01-22");
        }
    }
}

pub fn create_url_builder<P: AsRef<Path>>(path: P) -> Result<Vec<URLBuilder>> {
    let input = fs::read_to_string(path).expect("should exist condition.toml");
    let result: SearchConfig = toml::from_str(&input)?;

    let conditions = result.conditions.expect("should have field conditions");
    let result = conditions
        .into_iter()
        .map(|condition| {
            let one = "1".to_string();
            let squeeze = condition.squeeze.join(",");
            let max_charge = condition.max_charge.to_string();

            let today = Local::today();
            let checkin_date = condition.checkin.parse().unwrap();
            let (checkin_date_str, checkout_date_str) =
                if today.naive_local().cmp(&checkin_date) > Equal {
                    (condition.checkin, condition.checkout)
                } else {
                    let tommorow = today.succ();
                    (
                        tommorow.format("%F").to_string(),
                        tommorow.succ().format("%F").to_string(),
                    )
                };

            let queries = vec![
                ("middleClassCode", condition.prefecture),
                ("smallClassCode", condition.area),
                ("checkinDate", checkin_date_str),
                ("checkoutDate", checkout_date_str),
                ("adultNum", one),
                ("maxCharge", max_charge),
                ("squeezeCondition", squeeze),
            ];

            let mut url_builder = URLBuilder::new(VACANT_SEARCH_URL.to_string());
            url_builder.add_queries(&mut create_serach_condition(queries));
            url_builder
        })
        .collect();

    // assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
    Ok(result)
}

fn create_serach_condition<S, T>(input: Vec<(S, T)>) -> Vec<(String, String)>
where
    S: ToString,
    T: ToString,
{
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
