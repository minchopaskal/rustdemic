use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
#[derive(Debug)]
pub enum DiseaseKind {
    Red,
    Blue,
    Yellow,
    Black,
}

#[derive(Debug)]
pub struct Disease {
    pub spread: u8,
    pub kind: DiseaseKind,
}