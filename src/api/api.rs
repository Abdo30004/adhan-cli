use crate::api::error::AdhanError;
use crate::api::responses::PrayerTimesPeriod;

use super::responses::PrayerTimesResponse;
use reqwest;

pub async fn get_prayer_data_by_city(
    city: &str,
    period: PrayerTimesPeriod,
) -> Result<PrayerTimesResponse, AdhanError> {
    let url = format!(
        "https://muslimsalat.com/{city}/{period}.json?key=4cac3d04afdb6b23b8306e00d370bdd3"
    );
    println!("URL={}", url);

    let client= reqwest::Client::builder().user_agent(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"
    ).build()?;

    let response = client.get(url).send().await?;
    println!("got data");
    let data = response.json::<PrayerTimesResponse>().await?;
    Ok(data)
}
