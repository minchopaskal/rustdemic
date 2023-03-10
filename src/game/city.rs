use serde::{Serialize, Deserialize};

use super::disease::Disease;

pub type CityIdx = usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub disease: Disease,
    pub index: CityIdx,
}
