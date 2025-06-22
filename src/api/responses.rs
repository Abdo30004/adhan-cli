use crate::api::error::AdhanError;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::parsed::{ParsedLocation, ParsedPrayerTimeItem, ParsedPrayerTimesResponse};

#[derive(Clone, Serialize, Deserialize)]
pub enum PrayerTimesPeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl std::fmt::Display for PrayerTimesPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
            Self::Yearly => "yearly",
        };

        write!(f, "{}", data)
    }
}

impl FromStr for PrayerTimesPeriod {
    type Err = AdhanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "daily" => Ok(Self::Daily),
            "weekly" => Ok(Self::Weekly),
            "monthly" => Ok(Self::Monthly),
            "yearly" => Ok(Self::Yearly),
            _ => Err(AdhanError::InvalidPeriod),
        }
    }
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrayerTimeItem {
    pub date_for: String,
    pub fajr: String,
    pub shurooq: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub isha: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrayerTimesResponse {
    pub title: String,
    pub query: String,
    #[serde(rename = "for")]
    pub for_period: String,
    pub method: i32,
    pub prayer_method_name: String,
    pub daylight: String,
    pub timezone: String,
    pub map_image: String,
    pub sealevel: String,
    pub link: String,
    pub qibla_direction: String,
    pub latitude: String,
    pub longitude: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub country_code: String,
    pub items: Vec<PrayerTimeItem>,
    pub status_valid: i32,
    pub status_code: i32,
    pub status_description: String,
}

impl PrayerTimesResponse {
    pub fn parse(&self) -> Result<ParsedPrayerTimesResponse, AdhanError> {
        Ok(ParsedPrayerTimesResponse {
            items: self.parse_items()?,
            location: self.parse_location(),
            period: PrayerTimesPeriod::from_str(&self.for_period)?,
        })
    }

    pub fn parse_items(&self) -> Result<Vec<ParsedPrayerTimeItem>, AdhanError> {
        self.items.iter().map(|item| item.parse()).collect()
    }

    pub fn parse_location(&self) -> ParsedLocation {
        ParsedLocation {
            country: self.country.clone(),
            state: self.state.clone(),
            country_code: self.country_code.clone(),
            latitude: self.latitude.parse().ok(),
            longitude: self.longitude.parse().ok(),
            qibla_direction: self.qibla_direction.parse().ok(),
        }
    }
}

impl PrayerTimeItem {
    fn parse_prayer_time(str_prayer_time: &str) -> Result<NaiveTime, AdhanError> {
        Ok(NaiveTime::parse_from_str(str_prayer_time, "%I:%M %p")?)
    }

    fn parse_date(str_date: &str) -> Result<NaiveDate, AdhanError> {
        Ok(NaiveDate::parse_from_str(str_date, "%Y-%-m-%-d")?)
    }

    pub fn parse(&self) -> Result<ParsedPrayerTimeItem, AdhanError> {
        let parsed = ParsedPrayerTimeItem {
            date: Self::parse_date(&self.date_for)?,
            shurooq: Self::parse_prayer_time(&self.shurooq)?,
            fajr: Self::parse_prayer_time(&self.fajr)?,
            dhuhr: Self::parse_prayer_time(&self.dhuhr)?,
            asr: Self::parse_prayer_time(&self.asr)?,
            maghrib: Self::parse_prayer_time(&self.maghrib)?,
            isha: Self::parse_prayer_time(&self.isha)?,
        };

        Ok(parsed)
    }
}
