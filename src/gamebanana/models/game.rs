use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GBGame {
    pub row: usize,
    pub name: String,
    //pub mdate: usize,
}
