mod config;
mod messages;

use crate::config::AisMapConfig;
use crate::messages::position_report::PositionReport;
use futures_util::{SinkExt, StreamExt};
use mongodb::options::ClientOptions;
use serde_env::from_env;
use serde_json::{json, Value};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv()?;

    let configuration: AisMapConfig = from_env()?;

    let url = "wss://stream.aisstream.io/v0/stream";
    let client =
        mongodb::Client::with_options(ClientOptions::parse(configuration.mongodb_url).await?)?;

    let request = url.into_client_request()?;
    let (mut stream, _) = connect_async(request).await.unwrap();

    stream
        .send(create_subscription_message(&configuration.aisstream_apikey).into())
        .await?;

    while let Some(msg) = stream.next().await {
        let msg = msg?;

        if msg.is_binary() {
            let json: Value = serde_json::from_str(&msg.to_string())?;

            let position_report = process_message(json);
            match position_report {
                None => {}
                Some(position_report) => {
                    dbg!(&position_report);

                    let _ = client
                        .database("ais_map")
                        .collection::<PositionReport>("position_reports")
                        .insert_one(position_report)
                        .await;
                }
            }
        }
    }

    Ok(())
}

fn process_message(json: Value) -> Option<PositionReport> {
    let message_type = json.get("MessageType");

    return match message_type {
        None => {
            None
        }
        Some(value) => match value.as_str() {
            Some("PositionReport") => {
                let position_report: PositionReport =
                    serde_json::from_str(&json.to_string()).unwrap();

                Some(position_report)
            }
            _ => {
                None
            }
        },
    }
}

fn create_subscription_message(api_key: &str) -> String {
    return json!({
        "APIKey": api_key,
        "BoundingBoxes": [[[51.937240, 4.215410], [51.900218, 4.272807]]]
    })
    .to_string();
}
