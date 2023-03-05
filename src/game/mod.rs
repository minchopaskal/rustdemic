use std::{path::PathBuf, error::Error, default};

use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;

use crate::{util::{config::GameConfig, graph::Graph}, game::world::IMACT_RATE};

use self::{world::World, player::Player, turn::Turn, city::{City, CityIdx}, disease::Disease, cards::{PlayCard, Deck, NUM_EVENTS, EVENTS, DiseaseCard}};

pub mod turn;
pub mod city;
pub mod disease;
pub mod cards;
pub mod world;
pub mod player;

#[derive(Deserialize, Default, Debug)]
pub enum Difficulty {
    #[default]
    Easy,
    Normal,
    Hard,
}

#[derive(Default, Debug)]
pub struct Game {
    world: World,
    players: Vec<Player>,
    current_player: usize,
    turn: Turn,
    difficulty: Difficulty,

    end_game: bool,
}

impl Game {
    fn new() -> Self {
        Default::default()
    }

    pub fn from_file(yaml_file: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut game = Game::new();

        let cfg: GameConfig = serde_yaml::from_reader(std::fs::File::open(yaml_file)?)?;

        for (i, city) in cfg.cities.iter().enumerate() {
            game.world.cities.push(City {
                name: city.name.clone(),
                disease: Disease {
                    spread: 0,
                    kind: city.kind,
                },
                index: CityIdx(i),
            })
        }

        let num_cities = game.world.cities.len();
        game.world.map = Graph::new(num_cities);

        for pair in cfg.map.0.chunks(2) {
            if pair.len() != 2 {
                panic!("WARNING: map graph is not defined in pairs, maybe you are missing a value?");
            }
            game.world.map.connect(pair[0], pair[1]);
        }

        if let Some(d) = cfg.difficulty {
            game.difficulty = d;
        }

        let num_events = NUM_EVENTS;
        let num_epidemic_cards = match game.difficulty {
            Difficulty::Easy => 4,
            Difficulty::Normal => 5,
            Difficulty::Hard => 6,
        };
        let num_play_cards = num_cities + num_events + num_epidemic_cards;
        game.world.play_deck.cards.resize(num_play_cards, PlayCard::Epidemic);
        for i in 0..num_cities {
            game.world.play_deck.cards[i] = PlayCard::City(CityIdx(i));
        }
        for i in 0..num_events {
            game.world.play_deck.cards[i + num_cities] = PlayCard::Action(EVENTS[i].clone());
        }
        game.world.play_deck.cards_stack = (0..num_play_cards).collect();
        game.world.play_deck.cards_stack.shuffle(&mut thread_rng());

        let mut city_idx: usize = 0;
        game.world.disease_deck.cards.resize_with(num_cities, move || {
            let idx = city_idx;
            city_idx += 1;
            DiseaseCard(CityIdx(idx))
        });
        game.world.disease_deck.cards_stack = (0..num_cities).collect();
        game.world.disease_deck.cards_stack.shuffle(&mut thread_rng());

        Ok(game)
    }

    pub fn display(&self) {
        // TODO: clear screen better way
        print!("{}[2J", 27 as char);

        println!("Outbreaks: {}", self.world.outbreaks);
        println!("Impaction rate: {}({})", self.world.impaction_rate, IMACT_RATE[self.world.impaction_rate as usize]);
        for city in self.world.cities.iter() {
            print!("{} (Disease rate {}): ", city.name, city.disease.spread);

            let mut fst = true;
            for i in 0..self.world.cities.len() {
                if i == city.index.0 {
                    continue;
                }
                if self.world.map.connected(city.index.0, i) {
                    if fst {
                        print!("Can fly to [");
                        print!("{}", self.world.cities[i].name);
                        fst = false;
                    } else {
                        print!(", {}", self.world.cities[i].name);
                    }
                }
            }
            if !fst {
                print!("]");
            }
            println!();
        }
    }

    pub fn resolve_epidemic(&mut self, city: CityIdx) -> bool {
        true
    }

    pub fn disease_city(&mut self, city: CityIdx) -> bool{
        self.world.cities[city.0].disease.spread += 1;

        if self.world.cities[city.0].disease.spread > 3 {
            self.world.cities[city.0].disease.spread = 3;
            return self.resolve_epidemic(city);
        }

        false
    }

    // haha, but TODO: handle game end
    fn end_game(&mut self) {
        self.end_game = true;
        panic!("END GAME");
    }

    pub fn play_turn(&mut self) {
        let curr_player = &mut self.players[self.current_player];

        match &self.turn {
        Turn::Action(_) => {
            let action = curr_player.prompt_action();

            if let Some(turn) = self.turn.play_action() {
                println!("Next player {}'s turn!", &self.players[self.current_player].name);
                self.turn = turn;
            } else {
                panic!("Invalid state after calling Turn::play_action");
            }
        },
        Turn::Draw(_) => {
            let is_epidemic = if let Some(card) = self.world.play_deck.draw() {
                curr_player.add_card(card.clone());
                card == PlayCard::Epidemic
            } else {
                self.end_game();
                return;
            };

            if let Some(turn) = self.turn.draw_card(is_epidemic) {
                println!("Next player {}'s turn!", &self.players[self.current_player].name);
                self.turn = turn;
            } else {
                panic!("Invalid state after calling Turn::draw_card");
            }
        },
        Turn::Disease(_) => {
            // Draw disease card from stack
            if let Some(card) = self.world.disease_deck.draw() {
                println!("Disease card {} drawn!", self.world.cities[card.0.0].name);
                if self.disease_city(card.0) {
                    return;
                }
            } else {
                self.end_game();
                return;
            };
        },
        Turn::NextPlayer => {
            self.current_player = (self.current_player + 1) % self.players.len();
            if let Some(turn) = self.turn.advance_next_player() {
                println!("Next player {}'s turn!", &self.players[self.current_player].name);
                self.turn = turn;
            } else {
                panic!("Invalid state after calling Turn::advance_next_player");
            }
        },
        }
    }
}