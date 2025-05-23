use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/**
 * @todo make message_type enum
 * @todo flatten message property
 */

#[derive(Deserialize, Debug, Serialize)]
#[allow(dead_code)]
pub struct PositionReport {
    #[serde(rename = "Message")]
    message: Message,
    #[serde(rename = "MessageType")]
    message_type: String,
    #[serde(rename = "MetaData")]
    meta_data: super::meta_data::MetaData,
}

#[derive(Deserialize, Debug, Serialize)]
#[allow(dead_code)]
pub struct Message {
    #[serde(rename = "PositionReport")]
    position_report: PositionReportMessage,
}

#[derive(Deserialize, Debug, Serialize)]
#[allow(dead_code)]
struct PositionReportMessage {
    #[serde(rename = "Cog")]
    cog: f32,
    #[serde(rename = "CommunicationState")]
    communication_state: u32,
    #[serde(rename = "Latitude")]
    latitude: f32,
    #[serde(rename = "Longitude")]
    longitude: f32,
    #[serde(rename = "MessageID")]
    message_id: u32,
    #[serde(rename = "NavigationalStatus")]
    navigational_status: u32,
    #[serde(rename = "PositionAccuracy")]
    position_accuracy: bool,
    #[serde(rename = "Raim")]
    raim: bool,
    #[serde(rename = "RateOfTurn")]
    rate_of_turn: i32,
    #[serde(rename = "RepeatIndicator")]
    repeat_indicator: u32,
    #[serde(rename = "Sog")]
    sog: f64,
    #[serde(rename = "Spare")]
    spare: u32,
    #[serde(rename = "SpecialManoeuvreIndicator")]
    special_manoeuvre_indicator: u32,
    #[serde(rename = "Timestamp")]
    timestamp: u32,
    #[serde(rename = "TrueHeading")]
    true_heading: u32,
    #[serde(rename = "UserID")]
    user_id: u32,
    #[serde(rename = "Valid")]
    valid: bool,
}

impl TryFrom<Value> for PositionReport {
    type Error = PositionReportError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let message_type = value
            .get("MessageType")
            .ok_or(PositionReportError::MissingMessageType)?;

        match message_type.as_str() {
            Some("PositionReport") => {
                serde_json::from_value(value).map_err(PositionReportError::DeserializationError)
            }
            _ => Err(PositionReportError::InvalidMessageType),
        }
    }
}

#[derive(Debug)]
pub enum PositionReportError {
    MissingMessageType,
    InvalidMessageType,
    DeserializationError(serde_json::Error),
}

impl Error for PositionReportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PositionReportError::DeserializationError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for PositionReportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionReportError::MissingMessageType => write!(f, "MessageType is missing"),
            PositionReportError::InvalidMessageType => write!(f, "Invalid MessageType"),
            PositionReportError::DeserializationError(err) => {
                write!(f, "Deserialization error: {}", err)
            }
        }
    }
}
