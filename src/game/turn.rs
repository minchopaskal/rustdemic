#[derive(Debug)]
pub struct ActionsLeft(u8);

#[derive(Debug)]
pub struct DrawsLeft(u8);

#[derive(Debug)]
pub struct DiseasesLeft(u8);

#[derive(Debug)]
pub enum Turn {
    Action(ActionsLeft),
    Draw(DrawsLeft),
    Disease(DiseasesLeft),
    NextPlayer,
}


impl Default for Turn {
    fn default() -> Self {
        Self::Action(ActionsLeft(4))
    }
}

impl Turn {
    pub fn new() -> Self {
        Turn::default()
    }

    pub fn play_action(&self) -> Option<Turn> {
        match self {
        Turn::Action(left) => {
            if left.0 == 0 {
                None
            } else if left.0 == 1 {
                Some(Turn::Draw(DrawsLeft(2)))
            } else {
                Some(Turn::Action(ActionsLeft(left.0 - 1)))
            }
        },
        _ => None,
        }
    }

    pub fn draw_card(&self, is_epidemic: bool) -> Option<Turn> {
        match self {
        Turn::Draw(left) => {
            if left.0 == 0 {
                None
            } else if left.0 == 1 {
                Some(Turn::Disease(DiseasesLeft(2)))
            } else if is_epidemic {
                Some(Turn::Disease(DiseasesLeft(2)))
            } else {
                Some(Turn::Draw(DrawsLeft(left.0 - 1)))
            }
        },
        _ => None,
        }
    }

    pub fn spread_disease(&self) -> Option<Turn> {
        match self {
        Turn::Disease(left) => {
            if left.0 == 0 {
                None
            } else if left.0 == 1 {
                Some(Turn::NextPlayer)
            } else {
                Some(Turn::Disease(DiseasesLeft(left.0 - 1)))
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