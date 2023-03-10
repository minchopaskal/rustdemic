use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Debug)]
pub enum DiseaseKind {
    Red,
    Blue,
    Yellow,
    Black,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disease {
    pub spread: u8,
    pub kind: DiseaseKind,
}