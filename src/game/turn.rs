use serde::{Serialize, Deserialize};

pub type ActionsLeft = u8;
pub type DrawsLeft = u8;
pub type DiseasesLeft = u8;

#[derive(Debug, Serialize, Deserialize)]
pub enum Turn {
    Action(ActionsLeft),
    Draw(DrawsLeft),
    Disease(DiseasesLeft),
    PandemicInfect,
    PandemicIntensify,
    NextPlayer,
}


impl Default for Turn {
    fn default() -> Self {
        Self::Action(4)
    }
}

impl Turn {
    pub fn new() -> Self {
        Turn::default()
    }

    pub fn play_action(&self) -> Option<Turn> {
        match self {
        Turn::Action(left) => {
            if *left == 0 {
                None
            } else if *left == 1 {
                Some(Turn::Draw(2))
            } else {
                Some(Turn::Action(left - 1))
            }
        },
        _ => None,
        }
    }

    pub fn draw_card(&self, is_epidemic: bool) -> Option<Turn> {
        match self {
        Turn::Draw(left) => {
            if *left == 0 {
                None
            } else if *left == 1 {
                Some(Turn::Disease(2))
            } else if is_epidemic {
                Some(Turn::PandemicInfect)
            } else {
                Some(Turn::Draw(left - 1))
            }
        },
        _ => None,
        }
    }

    pub fn pandemic_infect(&self) -> Option<Turn> {
        match self {
        Turn::PandemicInfect => Some(Turn::PandemicIntensify),
        _ => None
        }
    }

    pub fn pandemic_intensify(&self) -> Option<Turn> {
        match self {
        Turn::PandemicIntensify => Some(Turn::Disease(2)),
        _ => None
        }
    }

    pub fn spread_disease(&self) -> Option<Turn> {
        match self {
        Turn::Disease(left) => {
            if *left == 0 {
                None
            } else if *left == 1 {
                Some(Turn::NextPlayer)
            } else {
                Some(Turn::Disease(left - 1))
            }
        },
        _ => None,
        }
    }

    pub fn advance_next_player(&self) -> Option<Turn> {
        match self {
        Turn::NextPlayer => Some(Self::new()),
        _ => None,
        }
    }
}