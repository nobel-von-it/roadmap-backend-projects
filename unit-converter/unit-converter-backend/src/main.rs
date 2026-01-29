use anyhow::Result;
use axum::{
    Form, Json, Router,
    http::{StatusCode, Uri},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

const INDEX: &str = include_str!("../../index.html");

trait Converter {
    fn value_to_base(&self, v: f64) -> f64;
    fn value_from_base(&self, v: f64) -> f64;
    fn convert(&self, other: &Self, v: f64) -> f64 {
        let v = self.value_to_base(v);
        other.value_from_base(v)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
enum LU {
    Mm,
    Cm,
    M,
    Km,
    I,
    F,
    Y,
    Ml,
}

impl TryFrom<&str> for LU {
    type Error = String;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "millimeter" => Ok(LU::Mm),
            "centimeter" => Ok(LU::Cm),
            "meter" => Ok(LU::M),
            "kilometer" => Ok(LU::Km),
            "inch" => Ok(LU::I),
            "foot" => Ok(LU::F),
            "yard" => Ok(LU::Y),
            "mile" => Ok(LU::Ml),
            _ => Err("invalid unit".to_string()),
        }
    }
}

impl Converter for LU {
    fn value_to_base(&self, value: f64) -> f64 {
        match self {
            LU::Mm => value / 1000.0,
            LU::Cm => value / 100.0,
            LU::M => value,
            LU::Km => value * 1000.0,
            LU::I => value * 0.0254,
            LU::F => value * 0.3048,
            LU::Y => value * 0.9144,
            LU::Ml => value * 1609.34,
        }
    }

    fn value_from_base(&self, value_in_meters: f64) -> f64 {
        match self {
            LU::Mm => value_in_meters * 1000.0,
            LU::Cm => value_in_meters * 100.0,
            LU::M => value_in_meters,
            LU::Km => value_in_meters / 1000.0,
            LU::I => value_in_meters / 0.0254,
            LU::F => value_in_meters / 0.3048,
            LU::Y => value_in_meters / 0.9144,
            LU::Ml => value_in_meters / 1609.34,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
enum WU {
    Mg,
    G,
    Kg,
    P,
    O,
}

impl TryFrom<&str> for WU {
    type Error = String;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "milligram" => Ok(WU::Mg),
            "gram" => Ok(WU::G),
            "kilogram" => Ok(WU::Kg),
            "pound" => Ok(WU::P),
            "ounce" => Ok(WU::O),
            _ => Err("invalid unit".to_string()),
        }
    }
}

impl Converter for WU {
    fn value_to_base(&self, value: f64) -> f64 {
        match self {
            WU::Mg => value / 1000000.,
            WU::G => value / 1000.,
            WU::Kg => value,
            WU::P => value * 0.453592,
            WU::O => value * 0.0283495,
        }
    }
    fn value_from_base(&self, value_in_kg: f64) -> f64 {
        match self {
            WU::Mg => value_in_kg * 1000000.,
            WU::G => value_in_kg * 1000.,
            WU::Kg => value_in_kg,
            WU::P => value_in_kg / 0.453592,
            WU::O => value_in_kg / 0.0283495,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
enum TU {
    C,
    F,
    K,
}

impl TryFrom<&str> for TU {
    type Error = String;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "celsius" => Ok(TU::C),
            "fahrenheit" => Ok(TU::F),
            "kelvin" => Ok(TU::K),
            _ => Err("invalid unit".to_string()),
        }
    }
}

impl Converter for TU {
    fn value_to_base(&self, value: f64) -> f64 {
        match self {
            TU::C => value + 273.15,
            TU::F => (value - 32.0) * 5.0 / 9.0 + 273.15,
            TU::K => value,
        }
    }
    fn value_from_base(&self, value_in_kelvin: f64) -> f64 {
        match self {
            TU::C => value_in_kelvin - 273.15,
            TU::F => (value_in_kelvin - 273.15) * 9.0 / 5.0 + 32.0,
            TU::K => value_in_kelvin,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
enum UnitConverter {
    Length { value: f64, from: LU, to: LU },
    Weight { value: f64, from: WU, to: WU },
    Temp { value: f64, from: TU, to: TU },
}

impl UnitConverter {
    fn any_from(convert_form: ConvertForm) -> Result<Self> {
        let value = convert_form.value;
        let from = convert_form.from;
        let to = convert_form.to;

        if let Ok(from) = LU::try_from(from.as_str()) {
            Ok(UnitConverter::Length {
                value,
                from,
                to: LU::try_from(to.as_str()).unwrap(),
            })
        } else if let Ok(from) = WU::try_from(from.as_str()) {
            Ok(UnitConverter::Weight {
                value,
                from,
                to: WU::try_from(to.as_str()).unwrap(),
            })
        } else if let Ok(from) = TU::try_from(from.as_str()) {
            Ok(UnitConverter::Temp {
                value,
                from,
                to: TU::try_from(to.as_str()).unwrap(),
            })
        } else {
            Err(anyhow::anyhow!("invalid query"))
        }
    }
    fn convert(self) -> f64 {
        match self {
            UnitConverter::Length { value, from, to } => from.convert(&to, value),
            UnitConverter::Weight { value, from, to } => from.convert(&to, value),
            UnitConverter::Temp { value, from, to } => from.convert(&to, value),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ConvertForm {
    value: f64,
    from: String,
    to: String,
}

async fn convert(Json(convert_form): Json<ConvertForm>) -> impl IntoResponse {
    println!("trying to convert {}", convert_form.value);

    match UnitConverter::any_from(convert_form) {
        Ok(converter) => {
            let value = converter.convert();
            (StatusCode::OK, Json(serde_json::json!({"value": value})))
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        ),
    }
}

async fn homepage() -> impl IntoResponse {
    Html(INDEX)
}

#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::new()
        .route("/api/convert", post(convert))
        .route("/", get(homepage))
        .nest_service("/static/scripts", ServeDir::new("static/scripts"));
    let listener = TcpListener::bind("127.0.0.1:3002").await?;

    axum::serve(listener, router).await?;
    Ok(())
}
