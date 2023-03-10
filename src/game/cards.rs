use serde::{Serialize, Deserialize};

use super::{city::CityIdx, disease::DiseaseKind, player::PlayerId};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Default, Clone)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DiseaseCard(pub CityIdx);

#[derive(PartialEq, Clone)]

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    BuildResearchCenter(CityIdx),
    TreatDisease(CityIdx),
    DiscoverCure(DiseaseKind),
    ShareKnowledge(PlayerId),

    Drive(CityIdx),
    Direct(CityIdx),
    Charter(CityIdx),
    Shuttle(CityIdx),
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, EnumIter)]
pub enum Event {
    GovermentGrant,
    Forecast,
    Airlift(CityIdx),
    OneQuietNight,
    ResilientPopulation(CardIdx),
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[derive(Debug)]
pub struct EventCard {
    pub title: &'static str, // yay, an actual correct use of 'static lifetime
    pub effect: &'static str,
    pub kind: Event,
}

pub const NUM_EVENTS: usize = 5;
pub static EVENTS: [EventCard; NUM_EVENTS] = [
    EventCard {
        title: "GovermentGrand",
        effect: "",
        kind: Event::GovermentGrant,
    },
    EventCard {
        title: "Forecast",
        effect: "",
        kind: Event::Forecast,
    },
    EventCard {
        title: "Airlift",
        effect: "",
        kind: Event::Airlift(0),
    },
    EventCard {
        title: "OneQuietNight",
        effect: "",
        kind: Event::OneQuietNight,
    },
    EventCard {
        title: "ResilientPopulation",
        effect: "",
        kind: Event::ResilientPopulation(0),
    },
];

#[derive(Default, PartialEq, Clone, Serialize, Deserialize)]
#[derive(Debug)]
pub enum PlayCard {
    City(CityIdx),
    Event(Event),
    #[default]
    Epidemic,
}

pub type CardIdx = usize;

#[derive(Default, Serialize, Deserialize)]
#[derive(Debug)]
pub struct Deck<T: Clone> {
    pub cards: Vec<T>,
    pub cards_stack: Vec<CardIdx>,
    pub cards_discard: Vec<CardIdx>,
}

impl<T: Clone> Deck<T> {
    pub fn draw(&mut self) -> Option<T> {
        if let Some(card_idx) = self.cards_stack.pop() {
            self.cards_discard.push(card_idx);

            Some(self.cards[card_idx].clone())
        } else {
            None
        }
    }
}