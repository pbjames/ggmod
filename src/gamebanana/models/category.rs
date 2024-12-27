use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GBModCategory {
    pub icon_url: String,
    pub name: String,
    pub id: usize,
}
