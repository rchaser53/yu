use serde_derive::Deserialize;
use toml;

use std::env;
use std::fs;
use std::io;
use std::path::Path;

use crate::url_builder::URLBuilder;

#[derive(Debug, Deserialize)]
struct SearchConfig {
    pub conditions: Option<Vec<Condition>>,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub target_url: String,
    pub squeeze: Vec<String>,
    pub prefecture: String,
    pub area: String,
    pub checkin: String,
    pub checkout: String,
    pub max_charge: usize,
}

pub fn create_url_builder<P: AsRef<Path>>(path: P) -> io::Result<Vec<URLBuilder>> {
    let input = fs::read_to_string(path).expect("should exist condition.toml");
    let result: SearchConfig = toml::from_str(&input)?;

    let conditions = result.conditions.expect("should have field conditions");
    let result = conditions
        .iter()
        .map(|condition| {
            let one = "1".to_string();
            let squeeze = condition.squeeze.join(",");
            let max_charge = condition.max_charge.to_string();
            let queries = vec![
                ("middleClassCode", &condition.prefecture),
                ("smallClassCode", &condition.area),
                ("checkinDate", &condition.checkin),
                ("checkoutDate", &condition.checkout),
                ("adultNum", &one),
                ("maxCharge", &max_charge),
                ("squeezeCondition", &squeeze),
            ];

            let mut url_builder = URLBuilder::new(condition.target_url.to_string());
            url_builder.add_queries(&mut create_serach_condition(queries));
            url_builder
        })
        .collect();
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
