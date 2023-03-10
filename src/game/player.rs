use serde::{Deserialize, Serialize};

use super::{city::CityIdx, cards::{PlayCard, Action}};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum Profession {
    #[default]
    None,
    
    Medic,
    Scientist,
    Dispatcher,
    Researcher,
    Ops,
    Contingency,
    QuarantineSpec,
}

pub type PlayerId = usize;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub cards: Vec<PlayCard>,
    pub profession: Profession,
    pub current_city: CityIdx,
    pub additional_actions: Vec<Action>,
    pub id: PlayerId,
}

impl Player {
    pub fn add_card(&mut self, card: PlayCard) {
        todo!();
    }

    pub fn prompt_action(&mut self) -> Action {
        Action::Direct(0)
    }
}
