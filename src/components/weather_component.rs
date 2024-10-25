use std::env;

use leptos::*;
use dotenv::dotenv;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Weather API Response Structures
#[derive(Deserialize)]
struct WeatherResponse {
    main: MainData,
    weather: Vec<WeatherInfo>,
    wind: WindData,
}

#[derive(Deserialize)]
struct MainData {
    temp: f64,
    humidity: u8,
}

#[derive(Deserialize)]
struct WeatherInfo {
    description: String,
}

#[derive(Deserialize)]
struct WindData {
    speed: f64,
}

// Application Data Structure
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WeatherData {
    temp: f64,
    description: String,
    humidity: u8,
    wind_speed: f64,
}

// Error Handling
#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Missing API key")]
    MissingApiKey,
}

#[component]
pub fn WeatherDashboard() -> impl IntoView {
    let (weather, set_weather) = create_signal(Option::<WeatherData>::None);
    let (error, set_error) = create_signal(Option::<String>::None);

    create_effect(move |_| {
        spawn_local(async move {
            match get_weather_data().await {
                Ok(data) => {
                    set_error.set(None);
                    set_weather.set(Some(data));
                }
                Err(e) => {
                    set_error.set(Some(e.to_string()));
                    set_weather.set(None);
                }
            }
        });
    });

    view! {
        <div class="weather">
            <h2>"Weather in Lagos"</h2>
            {move || {
                if let Some(err) = error.get() {
                    view! {
                        <div class="error">
                            <p>{format!("Error: {}", err)}</p>
                        </div>
                    }.into_view()
                } else {
                    match weather.get() {
                        Some(data) => view! {
                            <div class="weather-data">
                                <p>{format!("Temperature: {:.1}°C", data.temp)}</p>
                                <p>{data.description}</p>
                                <p>{format!("Humidity: {}%", data.humidity)}</p>
                                <p>{format!("Wind Speed: {:.1} m/s", data.wind_speed)}</p>
                            </div>
                        }.into_view(),
                        None => view! {
                            <div class="loading">
                                <p>"Loading..."</p>
                            </div>
                        }.into_view()
                    }
                }
            }}
        </div>
    }
}

pub async fn get_weather_data() -> Result<WeatherData, WeatherError> {
    dotenv().ok();
    // let api_key = std::env::var!("WEATHER_API_KEY").map_err(|_| WeatherError::MissingApiKey)?;
    let api_key = std::env::var("WEATHER_API_KEY").map_err(|_| WeatherError::MissingApiKey)?;
    
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q=Lagos&appid={}&units=metric",
        api_key
    );
    
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| WeatherError::NetworkError(e.to_string()))?;
        
    let weather_data: WeatherResponse = response
        .json()
        .await
        .map_err(|e| WeatherError::ParseError(e.to_string()))?;
    
    Ok(WeatherData {
        temp: weather_data.main.temp,
        description: weather_data.weather
            .first()
            .map(|w| w.description.clone())
            .unwrap_or_default(),
        humidity: weather_data.main.humidity,
        wind_speed: weather_data.wind.speed,
    })
}