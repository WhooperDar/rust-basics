mod lib;

use lib::Summarizable;

struct WeatherForecast { 
    high_temp: f64, 
    low_temp: f64, 
    chance_of_precipitation: f64, 
}

impl Summarizable for WeatherForecast { 
    fn summary(&self) -> String { 
        format!("The high will be {}, and the low will be {}. The chance of precipitation is {}%.", 
            self.high_temp, self.low_temp, self.chance_of_precipitation)
    }
}

pub fn weather(){
    let weather_forecast = WeatherForecast {
        high_temp: 64.4, 
        low_temp: 32.2, 
        chance_of_precipitation: 80.4,
    }; 

    println!("Weather forcase for today {}", weather_forecast.summary()); 
}

