use std::{collections::HashMap, f64::consts::PI as F64_PI};

use chrono::{NaiveDate, NaiveTime};

use crate::api::responses::PrayerTimesPeriod;

pub struct ParsedPrayerTimesResponse {
    pub location: ParsedLocation,
    pub items: Vec<ParsedPrayerTimeItem>,
    pub period: PrayerTimesPeriod,
}

#[derive(Debug, Clone)]

pub struct ParsedLocation {
    pub state: String,
    pub country: String,
    pub country_code: String,
    pub qibla_direction: Option<f64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ParsedPrayerTimeItem {
    pub date: NaiveDate,
    pub fajr: NaiveTime,
    pub shurooq: NaiveTime,
    pub dhuhr: NaiveTime,
    pub asr: NaiveTime,
    pub maghrib: NaiveTime,
    pub isha: NaiveTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Prayer {
    Fajr,
    Shurooq,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}

impl Prayer {
    pub fn name(&self) -> &'static str {
        match self {
            Prayer::Fajr => "Fajr",
            Prayer::Shurooq => "Shurooq",
            Prayer::Dhuhr => "Dhuhr",
            Prayer::Asr => "Asr",
            Prayer::Maghrib => "Maghrib",
            Prayer::Isha => "Isha",
        }
    }

    pub fn all_prayers() -> Vec<Prayer> {
        vec![
            Prayer::Fajr,
            Prayer::Shurooq,
            Prayer::Dhuhr,
            Prayer::Asr,
            Prayer::Maghrib,
            Prayer::Isha,
        ]
    }
}

impl ParsedPrayerTimeItem {
    pub fn get_prayer(&self, prayer: &Prayer) -> NaiveTime {
        match prayer {
            Prayer::Fajr => self.fajr,
            Prayer::Shurooq => self.shurooq,
            Prayer::Dhuhr => self.dhuhr,
            Prayer::Asr => self.asr,
            Prayer::Maghrib => self.maghrib,
            Prayer::Isha => self.isha,
        }
    }

    pub fn to_hash_map(&self) -> HashMap<Prayer, NaiveTime> {
        let mut map = HashMap::new();

        map.insert(Prayer::Fajr, self.fajr);
        map.insert(Prayer::Shurooq, self.shurooq);
        map.insert(Prayer::Dhuhr, self.dhuhr);
        map.insert(Prayer::Asr, self.asr);
        map.insert(Prayer::Maghrib, self.maghrib);
        map.insert(Prayer::Isha, self.isha);

        map
    }
}

impl ParsedLocation {
    fn longitude_to_tile(lon: f64, zoom: u32) -> u32 {
        let n = 2u32.pow(zoom) as f64;
        let x = ((lon + 180.0) / 360.0) * n;
        x.floor() as u32
    }
    fn latitude_to_tile(lat: f64, zoom: u32) -> u32 {
        let lat_rad = lat.to_radians(); // same as lat * PI / 180.0
        let n = 2u32.pow(zoom) as f64;

        let y = (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / F64_PI) / 2.0 * n;

        y.floor() as u32
    }
    pub fn get_tile(&self) -> Option<String> {
        if self.longitude.is_none() || self.latitude.is_none() {
            return None;
        }

        let longitude = self.longitude.unwrap();
        let latitude = self.latitude.unwrap();

        let zoom: u32 = 12;

        let x = Self::longitude_to_tile(longitude, zoom);

        let y = Self::latitude_to_tile(latitude, zoom);

        let link = format!("https://a.tile.openstreetmap.org/{zoom}/{x}/{y}.png");

        Some(link)
    }
}
