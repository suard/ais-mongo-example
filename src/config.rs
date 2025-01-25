use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct AisMapConfig {
    pub aisstream_apikey: String,
    pub aisstream_url: String,
    pub mongodb_url: String,
}
