use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FormCity {
    pub city: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CacheKey {
    pub city_name: String,
    pub api_timestamp: u64,
    pub user_timestamp: u64,
}

impl PartialOrd for CacheKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CacheKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.api_timestamp.cmp(&other.api_timestamp)
    }
}

impl CacheKey {
    pub fn new(city_name: String, api_timestamp: u64, user_timestamp: u64) -> CacheKey {
        CacheKey {
            city_name,
            api_timestamp,
            user_timestamp,
        }
    }
}

impl ToString for CacheKey {
    fn to_string(&self) -> String {
        format!(
            "{}-{}-{}",
            self.city_name, self.api_timestamp, self.user_timestamp
        )
    }
}

impl From<String> for CacheKey {
    fn from(s: String) -> Self {
        let parts = s.split("-").collect::<Vec<_>>();
        CacheKey {
            city_name: parts[0].to_string(),
            api_timestamp: parts[1].parse().unwrap(),
            user_timestamp: parts[2].parse().unwrap(),
        }
    }
}

// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub struct CacheKey {
//     pub city: String,
//     pub api_type: WeatherApiType,
//     pub units: WeatherUnits,
//     pub lang: WeatherLang,
//     pub bucket_ts: u64,
// }
// impl ToString for CacheKey {
//     fn to_string(&self) -> String {
//         format!(
//             "{}-{}-{}-{}-{}",
//             self.city,
//             self.api_type.to_string(),
//             self.units.to_string(),
//             self.lang.to_string(),
//             self.bucket_ts
//         )
//     }
// }
//
// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub enum WeatherLang {
//     En,
//     Ru,
// }
// impl ToString for WeatherLang {
//     fn to_string(&self) -> String {
//         match self {
//             WeatherLang::En => "en".to_string(),
//             WeatherLang::Ru => "ru".to_string(),
//         }
//     }
// }
// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub enum WeatherApiType {
//     Current,
// }
// impl ToString for WeatherApiType {
//     fn to_string(&self) -> String {
//         match self {
//             WeatherApiType::Current => "current".to_string(),
//         }
//     }
// }
// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub enum WeatherUnits {
//     Metric,
// }
// impl ToString for WeatherUnits {
//     fn to_string(&self) -> String {
//         match self {
//             WeatherUnits::Metric => "metric".to_string(),
//         }
//     }
// }

pub mod api {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PreparedTemp {
        pub temp: f32,
        pub temp_max: f32,
        pub temp_min: f32,
        pub humidity: f32,
        pub pressure: f32,
        pub wind_speed: f32,
    }

    impl PreparedTemp {
        pub fn new(
            temp: f32,
            temp_max: f32,
            temp_min: f32,
            humidity: f32,
            pressure: f32,
            wind_speed: f32,
        ) -> Self {
            Self {
                temp,
                temp_max,
                temp_min,
                humidity,
                pressure,
                wind_speed,
            }
        }
    }

    impl Display for PreparedTemp {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", serde_json::to_string(self).unwrap())
        }
    }

    impl From<String> for PreparedTemp {
        fn from(s: String) -> Self {
            serde_json::from_str(&s).unwrap()
        }
    }
}

pub mod vc {
    use serde::Deserialize;

    use super::api;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ResponseVC {
        address: String,
        current_conditions: CurrentConditionsVC,
        alerts: Vec<AlertVC>,
        days: Vec<DayVC>,
        description: String,

        latitude: f32,
        longitude: f32,

        query_cost: f32,
        resolved_address: String,
        timezone: String,
        tzoffset: f32,
    }

    impl ResponseVC {
        pub fn get_prepared_temp(&self) -> api::PreparedTemp {
            let current_day = &self.days[0];
            api::PreparedTemp::new(
                self.current_conditions.temp,
                current_day.tempmax,
                current_day.tempmin,
                self.current_conditions.humidity,
                self.current_conditions.pressure,
                self.current_conditions.windspeed,
            )
        }
        pub fn get_daily_forecase(&self) -> &[DayVC] {
            &self.days
        }
        pub fn get_current_api_time(&self) -> u64 {
            self.current_conditions.datetime_epoch
        }
        pub fn get_current_timestamp(&self) -> u64 {
            let cc = &self.current_conditions;
            let cd = &self.days[0];
            if cc.datetime_epoch != cd.datetime_epoch {
                println!(
                    "warning: current_conditions.datetime_epoch ({}) != days[0].datetime_epoch ({})",
                    cc.datetime_epoch, cd.datetime_epoch
                );
                println!(
                    "warning: current_conditions.datetime ({}) != days[0].datetime ({})",
                    &cc.datetime, &cd.datetime
                );
            }
            self.current_conditions
                .datetime_epoch
                .max(self.days[0].datetime_epoch)
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CurrentConditionsVC {
        cloudcover: f32,
        conditions: String,
        datetime: String,
        datetime_epoch: u64,
        dew: f32,
        feelslike: f32,
        humidity: f32,
        icon: String,
        moonphase: f32,
        pressure: f32,
        snow: f32,
        snowdepth: f32,
        solarenergy: f32,
        solarradiation: f32,
        source: String,
        stations: Option<Vec<String>>,
        sunrise: String,
        sunrise_epoch: u64,
        sunset: String,
        sunset_epoch: u64,
        temp: f32,
        uvindex: f32,
        visibility: Option<f32>,
        winddir: f32,
        windgust: Option<f32>,
        windspeed: f32,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AlertVC {
        description: String,
        ends: String,
        ends_epoch: u64,

        event: String,
        headline: String,

        id: String,
        language: String,
        link: String,
        onset: String,
        onset_epoch: u64,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DayVC {
        cloudcover: f32,
        conditions: String,
        datetime: String,
        datetime_epoch: u64,
        dew: f32,
        feelslike: f32,
        feelslikemax: f32,
        feelslikemin: f32,
        hours: Vec<HourVC>,
        humidity: f32,
        icon: String,
        moonphase: f32,
        pressure: f32,
        severerisk: f32,
        snow: f32,
        snowdepth: f32,
        solarenergy: f32,
        solarradiation: f32,
        source: String,
        stations: Option<Vec<String>>,
        sunrise: String,
        sunrise_epoch: u64,
        sunset: String,
        sunset_epoch: u64,
        temp: f32,
        tempmax: f32,
        tempmin: f32,
        uvindex: f32,
        visibility: f32,
        winddir: f32,
        windgust: f32,
        windspeed: f32,
    }

    impl DayVC {
        pub fn get_temps(&self) -> (f32, f32, f32) {
            (self.temp, self.tempmax, self.tempmin)
        }
        pub fn get_date(&self) -> &str {
            &self.datetime
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HourVC {
        cloudcover: f32,
        conditions: String,
        datetime: String,
        datetime_epoch: u64,
        dew: f32,
        temp: f32,
        feelslike: f32,
        humidity: f32,
        icon: String,
        pressure: f32,
        severerisk: f32,
        snow: f32,
        snowdepth: f32,
        solarenergy: f32,
        solarradiation: f32,
        source: String,
        stations: Option<Vec<String>>,
        uvindex: f32,
        visibility: f32,
        winddir: f32,
        windgust: f32,
        windspeed: f32,
    }
}
