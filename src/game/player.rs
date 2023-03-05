use super::{city::CityIdx, cards::{PlayCard, ActionCard}};

#[derive(Debug)]
pub enum Profession {
    Medic,
    Scientist,
    // add more
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub cards: Vec<PlayCard>,
    pub profession: Profession,
    pub current_city: CityIdx,
    pub additional_actions: Vec<ActionCard<'static>>,
}

impl Player {
    pub fn add_card(&mut self, card: PlayCard) {
        todo!();
    }

    pub fn prompt_action(&mut self) {

    }
}
