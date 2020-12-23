use std::collections::HashMap;
use serde::Deserialize;
pub mod dbmodels {

}

pub mod request_models {
   use super::*;
   #[derive(Deserialize)]
   pub struct HostnameProtocol {
      pub data: Vec<HashMap<String, String>>
   }
   #[derive(Deserialize)]
   pub struct HostnameMuchData {
      pub data: Vec<MuchData>
   }
   #[derive(Deserialize, Debug)]
   pub struct MuchData {
      pub full_link: String,
      pub link_only: String,
      pub protocol: String,
      pub hostname: String,
      pub full_path: String,
      pub path_only: String,
      pub params: String,
      pub page_from: String,
   }
}