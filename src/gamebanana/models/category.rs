use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GBModCategory {
    pub row: usize,
    pub icon_url: String,
    pub name: String,
}
