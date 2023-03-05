use super::city::CityIdx;


#[derive(Default, Clone)]
#[derive(Debug)]
pub struct DiseaseCard(pub CityIdx);

#[derive(PartialEq, Clone)]

#[derive(Debug)]
pub enum Action {
    GovermentGrant,
    Forecast,
    Airlift,
    OneQuietNight,
    ResilientPopulation,

    BuildResearchCenter,
    TreatDisease,
    DiscoverCure,
    ShareKnowledge,

    Drive,
    Direct,
    Charter,
    Shuttle,
}

#[derive(PartialEq, Clone)]
#[derive(Debug)]
pub struct ActionCard<'a> {
    pub title: &'a str,
    pub effect: &'a str,
    pub kind: Action,
}

pub const NUM_EVENTS: usize = 5;
pub const EVENTS: [ActionCard; NUM_EVENTS] = [
    ActionCard {
        title: "GovermentGrand",
        effect: "",
        kind: Action::GovermentGrant,
    },
    ActionCard {
        title: "Forecast",
        effect: "",
        kind: Action::Forecast,
    },
    ActionCard {
        title: "Airlift",
        effect: "",
        kind: Action::Airlift,
    },
    ActionCard {
        title: "OneQuietNight",
        effect: "",
        kind: Action::OneQuietNight,
    },
    ActionCard {
        title: "ResilientPopulation",
        effect: "",
        kind: Action::ResilientPopulation,
    },
];

#[derive(Default, PartialEq, Clone)]
#[derive(Debug)]
pub enum PlayCard {
    City(CityIdx),
    Action(ActionCard<'static>),
    #[default]
    Epidemic,
}

#[derive(Default)]
#[derive(Debug)]
pub struct Deck<T: Clone> {
    pub cards: Vec<T>,
    pub cards_stack: Vec<usize>,
    pub cards_discard: Vec<usize>,
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