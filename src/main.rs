use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use ruuvi_sensor_protocol::{Humidity, ParseError, Pressure, SensorValues, Temperature};
use std::error::Error;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::time;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    message: String,
}

fn from_manufacturer_data(data: &[u8]) -> Result<SensorValues, ParseError> {
    if data.len() > 2 {
        SensorValues::from_manufacturer_specific_data(1177, data)
    } else {
        Err(ParseError::EmptyValue)
    }
}

async fn find_ruuvi_peripheral(central: &Adapter) -> Option<(Peripheral, Vec<u8>)> {
    match central.peripherals().await {
        Ok(peripherals) => {
            for p in peripherals {
                let data = p.properties().await;
                match data {
                    Ok(Some(properties)) => {
                        // manufacturerdata 0x99 0x04
                        if let Some((1177, data)) = properties.manufacturer_data.iter().next() {
                            return Some((p, data.clone()));
                        }
                    }
                    Err(e) => eprintln!("unable to get peripheral properties: {e:?}"),
                    _ => {}
                }
            }
        }
        Err(e) => eprintln!("unable to enumerate peripherals: {e:?}"),
    }
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let central = manager
        .adapters()
        .await
        .expect("Unable to fetch adapter list.")
        .into_iter()
        .nth(0)
        .expect("Unable to find adapters.");

    central.start_scan(ScanFilter::default()).await?;

    time::sleep(Duration::from_secs(2)).await;

    loop {
        if let Some((ruuvi_peripheral, data)) = find_ruuvi_peripheral(&central).await {
            let sensor_values = from_manufacturer_data(&data);
            match sensor_values {
                Ok(sensor_values) => {
                    // Here we can process our data
                    let millicelsius = sensor_values
                        .temperature_as_millicelsius()
                        .unwrap_or_default();
                    let pascals = sensor_values.pressure_as_pascals().unwrap_or_default();
                    let humidity = sensor_values.humidity_as_ppm().unwrap_or_default();

                    let message = format!(
                        "{}: {:>6.1}°C {:>8.1}hPa {:>6.1}%H",
                        ruuvi_peripheral.address(),
                        millicelsius as f32 / 1000.0,
                        pascals as f32 / 100.0,
                        humidity as f32 / 10000.0
                    );
                    println!("{}", message);

                    // Send HTTP POST message to server
                    let post = Post { message };
                    if let Err(e) = reqwest::Client::new()
                        .post("http://95.216.207.125:9000/api/chats")
                        .json(&post)
                        .send()
                        .await
                    {
                        eprintln!("failed to POST: {e:?}");
                    }
                }
                Err(e) => eprintln!("failed to parse ruuvi data: {e:?}"),
            }
        }
        time::sleep(Duration::from_secs(1)).await;
    }
}
