mod config;
mod messages;

use crate::config::AisMapConfig;
use crate::messages::position_report::PositionReport;
use futures_util::{SinkExt, StreamExt};
use log::{debug, info, error};
use mongodb::options::ClientOptions;
use serde_env::from_env;
use serde_json::{json, Value};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv()?;
    env_logger::init();

    info!("Starting AIS Map Service");

    let configuration: AisMapConfig = from_env()?;

    let client =
        mongodb::Client::with_options(ClientOptions::parse(configuration.mongodb_url).await?)?;

    let request = configuration.aisstream_url.into_client_request()?;

    let mut stream;

    match connect_async(request).await {
        Ok((ws_stream, _)) => {
            println!("Connected successfully!");
            stream = ws_stream;
        }
        Err(Error::Http(response)) => {
            if let Some(body) = response.body() {
                let body_str = String::from_utf8_lossy(body);
                eprintln!(
                    "Connection failed: HTTP {} - {}",
                    response.status(),
                    body_str
                );
            } else {
                eprintln!("Connection failed: HTTP {}", response.status());
            }

            return Err(Error::Http(response).into());
        }
        Err(err) => {
            eprintln!("Connection failed: {:?}", err);
            return Err(err.into());
        }
    }

    stream
        .send(create_subscription_message(&configuration.aisstream_apikey).into())
        .await?;

    info!("Initialized aisstream websocket client");

    while let Some(msg) = stream.next().await {
        let msg = msg?;
        if msg.is_binary() {
            let json: Value = serde_json::from_str(&msg.to_string())?;

            let position_report = match PositionReport::try_from(json) {
                Ok(report) => report,
                Err(e) => {
                    debug!("Invalid position report: {}", e);
                    continue;
                }
            };

            debug!("{:?}", position_report);

            if let Err(e) = client
                .database("ais_map")
                .collection::<PositionReport>("position_reports")
                .insert_one(position_report)
                .await
            {
                error!("Failed to insert into database: {}", e);
            } else {
                info!("Received position report message and stored it in the database (mongodb)");
            }
        }
    }

    Ok(())
}

fn create_subscription_message(api_key: &str) -> String {
    json!({
        "APIKey": api_key,
        "BoundingBoxes": [[[51.937240, 4.215410], [51.900218, 4.272807]]]
    })
    .to_string()
}
