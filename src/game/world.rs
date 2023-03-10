use serde::{Deserialize, Serialize};

use crate::util::graph::Graph;

use super::{city::City, cards::{Deck, PlayCard, DiseaseCard}};

pub(crate) const IMACT_RATE: [u8; 8] = [2, 2, 2, 3, 3, 3, 4, 4];

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct World {
    pub map: Graph,
    pub cities: Vec<City>,
    pub play_deck: Deck<PlayCard>,
    pub disease_deck: Deck<DiseaseCard>,
    pub outbreaks: u8,
    pub impaction_rate: u8
}