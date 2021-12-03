use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use tokio::sync::oneshot;

use crate::message::*;

pub fn retrieve_weather(tx: MessageSender, query: &str) -> OneshotReceiver<WeatherResponse> {
    let query = format!("https://wttr.in/{}?format=j1", query);
    let (send, recv) = oneshot::channel();
    let _ = tx.send(Message::FetchNewResource { url: query, send });
    recv
}

pub async fn fetch_weather(tx: OneshotSender<WeatherResponse>, url: String) -> Result<()> {
    let resp = reqwest::get(url).await?.text().await?;
    let weather_response: WeatherResponse = serde_json::from_str(&resp)?;
    let _ = tx.send(weather_response);
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct WeatherResponse {
    #[serde(deserialize_with = "de_first_of_arr")]
    pub current_condition: CurrentCondition,

    #[serde(deserialize_with = "de_first_of_arr")]
    pub nearest_area: NearestArea,
}

#[derive(Serialize, Deserialize)]
pub struct CurrentCondition {
    #[serde(rename = "FeelsLikeC", deserialize_with = "de_from_str")]
    pub feels_like: f64,

    #[serde(rename = "temp_C", deserialize_with = "de_from_str")]
    pub temperature: f64,

    #[serde(rename = "cloudcover", deserialize_with = "de_from_str")]
    pub cloud_cover: f64,

    #[serde(deserialize_with = "de_from_str")]
    pub humidity: f64,

    #[serde(rename = "weatherDesc", deserialize_with = "de_first_of_value_arr")]
    pub weather_description: String,

    #[serde(rename = "localObsDateTime")]
    pub last_update_local_time: String,
}

#[derive(Serialize, Deserialize)]
pub struct NearestArea {
    #[serde(deserialize_with = "de_first_of_value_arr")]
    pub country: String,

    #[serde(deserialize_with = "de_first_of_value_arr")]
    pub region: String,

    #[serde(rename = "areaName", deserialize_with = "de_first_of_value_arr")]
    pub area_name: String,
}

// Original code from: https://users.rust-lang.org/t/serde-deserialize-string-field-in-json-to-a-different-type/12942
fn de_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

fn de_first_of_arr<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Vec::<T>::deserialize(deserializer)?.swap_remove(0))
}

/// A "value array" is an array of objects which only contain a "value" field
/// Example:
/// ```json
/// {
///   "country": [
///     {
///       "value" = "United States of America"
///     }
///   ]
/// }
/// ```
fn de_first_of_value_arr<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    struct Inner<T> {
        value: T,
    }

    Ok(Vec::<Inner<T>>::deserialize(deserializer)?
        .swap_remove(0)
        .value)
}
