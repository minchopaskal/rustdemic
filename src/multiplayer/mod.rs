use crate::game::player::{PlayerId, Player};

pub mod client;
pub mod server;

type MsgHash = u128;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerMsg {
    DrawCard,
    DrawDiseaseCard,
    DrawEpidemicCard,
    PromptAction(MsgHash),
    PromptEvent(MsgHash),

    Started(MsgHash),
    AddPlayer(Vec<Player>),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClientMsg {
    Success(MsgHash),

    Id(String),

    Start(MsgHash),

    CardDiscarded,
    Move,
    Treat,
    Cure,
    Build,
    Event,
    RejectEvent(MsgHash),
}