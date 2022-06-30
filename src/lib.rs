use std::error;
use std::result::Result;

#[derive(Debug, Deserialize)]
pub struct CountryPrediction {
    pub country_id: String,
    pub probability: f32,
}

#[derive(Debug, Deserialize)]
pub struct NamePredictions {
    pub name : String,
    pub country: Vec<CountryPrediction>
}


pub async fn fetch_data(name : &str) -> Result<String, Box<dyn error::Error>>{
    let  mut url = String::from("https://api.nationalize.io/?name=");
    url.push_str(name);
    let body = reqwest::get(url)
    .await?
    .text()
    .await?;
    Ok(body)

}

use serde::Deserialize;
use serde_json;

pub fn typed_example(text : &str) -> serde_json::Result<()> {
    let p: NamePredictions = serde_json::from_str(text)?;
    let possible_countries = p.country;

    if possible_countries.len() == 0 {
        println!("Hi {:?}. I can't seem to figure out which country you're from.", p.name);
    } else {
        println!("Hi {:?}. I think you are from one of the following countries:", p.name);
        for (index, c) in possible_countries.iter().enumerate() {
            println!("{} . {} ", index+1, c.country_id);
        }
    }
    
    Ok(())
}