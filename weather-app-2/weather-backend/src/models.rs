use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FormCity {
    pub city: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CacheKey {
    pub city: String,
    pub api_type: WeatherApiType,
    pub units: WeatherUnits,
    pub lang: WeatherLang,
    pub bucket_ts: u64,
}
impl ToString for CacheKey {
    fn to_string(&self) -> String {
        format!(
            "{}-{}-{}-{}-{}",
            self.city,
            self.api_type.to_string(),
            self.units.to_string(),
            self.lang.to_string(),
            self.bucket_ts
        )
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WeatherLang {
    En,
    Ru,
}
impl ToString for WeatherLang {
    fn to_string(&self) -> String {
        match self {
            WeatherLang::En => "en".to_string(),
            WeatherLang::Ru => "ru".to_string(),
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WeatherApiType {
    Current,
}
impl ToString for WeatherApiType {
    fn to_string(&self) -> String {
        match self {
            WeatherApiType::Current => "current".to_string(),
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WeatherUnits {
    Metric,
}
impl ToString for WeatherUnits {
    fn to_string(&self) -> String {
        match self {
            WeatherUnits::Metric => "metric".to_string(),
        }
    }
}

// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub struct CacheKey {
//     pub city: String,
//     pub timestamp: u64,
// }

pub mod api {
    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
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
