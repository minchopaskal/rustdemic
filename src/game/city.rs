use super::disease::Disease;

#[derive(Default, Clone, PartialEq)]
#[derive(Debug)]
pub struct CityIdx(pub usize);

#[derive(Debug)]
pub struct City {
    pub name: String,
    pub disease: Disease,
    pub index: CityIdx,
}
