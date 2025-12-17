use serde::Deserialize;
use std::env;
use dotenvy::dotenv;

// --- ESTRUCTURAS DE DATOS ---
// La API devuelve un JSON complejo. Solo necesitamos capturar lo que nos importa.
// Estructura del JSON de OpenWeather:
// {
//    "weather": [ { "description": "clear sky", ... } ],
//    "main": { "temp": 298.5, "humidity": 80, ... },
//    "name": "London"
// }

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<WeatherDetail>,
    main: MainData,
    name: String,
}

#[derive(Deserialize, Debug)]
struct WeatherDetail {
    description: String,
}

#[derive(Deserialize, Debug)]
struct MainData {
    temp: f64,
    humidity: u8,
}

// --- CONSTANTES ---
const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

// --- LOGICA ---
async fn get_weather(city: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    // 1. Construir la URL con los parámetros necesarios
    // La URL debe verse así: ...?q=London&units=metric&appid=TU_KEY
    let url = format!("{}?q={}&units=metric&appid={}", BASE_URL, city, api_key);

    // 2. Hacer la petición (Aquí ocurre la magia asíncrona)
    // TODO:
    // Usa reqwest::get(&url).await?
    // Luego usa .json::<WeatherResponse>().await? sobre la respuesta
    // Nota: El '?' propaga el error si falla la conexión o el parseo.
    let response = reqwest::get(&url).await?
        .json::<WeatherResponse>().await?;

    Ok(response) // Sin punto y coma = return
}

// "tokio::main" es una macro que prepara el programa para ejecutar código asíncrono
#[tokio::main]
async fn main() {
    
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY is not defined");
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: cargo run <ciudad>");
        return;
    }

    let city = &args[1];
    println!("Consultando el clima para {}...", city);

    // Llamamos a la función asíncrona.
    // Como es asíncrona, devuelve un "Futuro", hay que esperarlo con .await
    match get_weather(city, &api_key).await {
        Ok(response) => {
            // TODO: Imprime los datos bonitos.
            // Ej: "Ciudad: Madrid"
            //     "Temperatura: 25.5°C"
            //     "Descripción: cielo claro" (Ojo: weather es un Vector, accede al primer elemento)
            //     "Humedad: 60%"
            println!("City: {}", response.name); // Debug temporal
            println!("Temperature: {}°C", response.main.temp);
            println!("Description: {}", response.weather[0].description);
            println!("Humidity: {}%", response.main.humidity);
        }
        Err(e) => {
            println!("Error al obtener el clima: {}", e);
        }
    }
}
