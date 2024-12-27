use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GBGame {
    pub name: String,
    pub developer: String,
    pub date: usize,
}
