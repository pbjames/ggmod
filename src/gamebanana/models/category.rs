use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GBCategory {
    pub icon_url: String,
    pub name: String,
}
