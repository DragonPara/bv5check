extern crate reqwest;
use serde_json::Value;
use std::{error::Error, io::Write};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api1 = String::from("https://api.gametools.network/bfv/stats/?format_values=true&name=");
    loop {
        let mut name = String::new();
        print!("user: ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut name).unwrap();
        let response = reqwest::get(api1.clone() + &name).await?.text().await?;
        let json_value: Value = serde_json::from_str(&response).expect("Invalid JSON");

        println!("Level:     {}", json_value["rank"]);
        println!("KD:        {}", json_value["killDeath"]);
        println!("KPM:       {}", json_value["killsPerMinute"]);
        println!("SPM:       {}", json_value["scorePerMinute"]);
        println!("杀人数:    {}", json_value["kills"]);
        println!("救人数:    {}", json_value["revives"]);
        println!("游戏时间:  {}", json_value["timePlayed"].as_str().unwrap());
        let api2 = String::from(
            "https://api.gametools.network/bfv/weapons/?format_values=true&lang=zh-cn&name=",
        );
        let response = reqwest::get(api2.clone() + &name).await?.text().await?;
        let json_value: Value = serde_json::from_str(&response).expect("Invalid JSON");

        let weapons = &json_value["weapons"];
        let weapons: &mut Vec<Value> = &mut weapons.as_array().unwrap().clone();
        weapons.sort_by(|a, b| {
            b["kills"]
                .as_i64()
                .unwrap()
                .cmp(&a["kills"].as_i64().unwrap())
        });
        println!("擅长武器:\n------------------------");
        for i in 0..3 {
            println!(
                "{}\n\t击杀数:{:7} KPM:{:5.2} 爆头率:{}",
                weapons[i]["weaponName"].as_str().unwrap(),
                weapons[i]["kills"].as_i64().unwrap(),
                weapons[i]["killsPerMinute"].as_f64().unwrap(),
                weapons[i]["headshots"].as_str().unwrap()
            );
        }
        let api3 = String::from(
            "https://api.gametools.network/bfv/vehicles/?format_values=true&lang=zh-cn&name=",
        );
        let response = reqwest::get(api3.clone() + &name).await?.text().await?;
        let json_value: Value = serde_json::from_str(&response).expect("Invalid JSON");
        let vehicles = &json_value["vehicles"];
        let vehicles: &mut Vec<Value> = &mut vehicles.as_array().unwrap().clone();
        vehicles.sort_by(|a, b| {
            b["kills"]
                .as_i64()
                .unwrap()
                .cmp(&a["kills"].as_i64().unwrap())
        });
        println!("擅长载具:\n------------------------");
        for i in 0..3 {
            println!(
                "{}\n\t击杀数:{:7} KPM:{:5.2} 摧毁数:{:7}",
                vehicles[i]["vehicleName"].as_str().unwrap(),
                vehicles[i]["kills"].as_i64().unwrap(),
                vehicles[i]["killsPerMinute"].as_f64().unwrap(),
                vehicles[i]["destroyed"].as_i64().unwrap()
            );
        }
    }
}
