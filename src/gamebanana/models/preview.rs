use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBPreviewMedia {
    pub base_url: String,
    pub file: String,
}
