use crate::api::error::AdhanError;
use crate::api::responses::PrayerTimesPeriod;

use super::responses::PrayerTimesResponse;
use regex;
use reqwest;
use serde;

pub async fn get_prayer_data_by_city(
    city: &str,
    period: PrayerTimesPeriod,
) -> Result<PrayerTimesResponse, AdhanError> {
    let url = format!("https://muslimsalat.com/{city}/{period}.json");
    println!("URL={}", url);

    let client= reqwest::Client::builder().user_agent(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"
    ).build()?;

    let response = client.get(url).send().await?;

    let response_text = response.text().await?;

    let regex_m = regex::Regex::new(r#""-?(\d+(?:\.\d+)?)""#).unwrap(); // fix problem of api response types

    let response_text = regex_m.replace_all(&response_text, "$1");

    println!("{response_text}");
    let data: PrayerTimesResponse = serde_json::from_str(&response_text).unwrap();
    Ok(data)
}
