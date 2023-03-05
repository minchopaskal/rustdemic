use serde::Deserialize;

use crate::game::{disease::DiseaseKind, Difficulty};

#[derive(Deserialize)]
pub struct GraphDef(pub Vec<usize>);

#[derive(Deserialize)]
pub struct CityDef {
    pub name: String,
    pub kind: DiseaseKind,
}

#[derive(Deserialize)]
pub struct GameConfig {
    pub cities: Vec<CityDef>,
    pub map: GraphDef,
    pub difficulty: Option<Difficulty>,
}