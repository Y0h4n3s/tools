use serde_derive::*;
#[derive(Deserialize, Debug)]
pub struct WaybackData {
    pub data: Vec<Vec<String>>,
}