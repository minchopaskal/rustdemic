use std::{path::PathBuf, collections::HashMap, net::{SocketAddr, IpAddr, Ipv4Addr}, default, sync::{Arc, Mutex}, thread::JoinHandle};

use message_io::{node::{NodeHandler, NodeListener, self, NodeEvent, NodeTask}, network::{Endpoint, Transport, NetEvent}};

use crate::game::{Game, player::{Player, PlayerId, self}};

use super::{MsgHash, ServerMsg, ClientMsg};

enum Signal {
    SendStart,
}

#[derive(Default)]
pub struct Server {
    pub game: Game,

    // Mapping Player <-> Endpoint
    clients: HashMap<Endpoint, PlayerId>,

    handler: Option<NodeHandler<Signal>>,
    listener: Option<NodeListener<Signal>>,
    //node_task: Option<NodeTask>,
    node_task: Option<JoinHandle<()>>,

    addr: Option<SocketAddr>,

    requests: HashMap<MsgHash, ServerMsg>,

    started: bool,
}

#[derive(Debug)]
pub enum ServerError {
    NotConnected
}

impl Server {
    pub fn new(port: &str) -> Option<Self> {
        let (handler, listener) = node::split();

        if let Ok(game) = Game::from_file(PathBuf::from("assets/config.yaml")) {
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port.parse::<u16>().unwrap());
            return Some(Self {
                game,
                clients: HashMap::new(),
                handler: Some(handler),
                listener: Some(listener),
                node_task: None,
                addr: Some(addr),
                requests: HashMap::new(),
                started: false,
            })
        }
        None
    }

    fn update_players(&self) {
        let add_player_annon = ServerMsg::AddPlayer(self.game.players.clone());
        let data = bincode::serialize(&add_player_annon).unwrap();
        for client_endpoint in self.clients.keys() {
            self.handler.as_ref().unwrap().network().send(*client_endpoint, &data);
        }
    }

    pub fn stop(&mut self) {
        if self.handler.is_none() {
            return
        }

        self.handler.as_ref().unwrap().stop();
    }

    pub fn players_map<F>(&self, f: F) where F: Fn(&Player) {
        for player in self.game.players.iter() {
            f(player);
        }
    }

    fn set_join_handle(&mut self, jh: JoinHandle<()>) {
        self.node_task = Some(jh);
    }
}

pub fn start_server(server: Arc<Mutex<Server>>) {
    println!("Starting server...");

    let server_orig = server;
    let mut server = server_orig.lock().unwrap();

    if let Err(_) = server.handler.as_ref().unwrap().network().listen(Transport::FramedTcp, &server.addr.unwrap()) {
        println!("Error while starting server listen!");
        return
    }

    let listener = server.listener.take().unwrap();

    let server = server_orig.clone();
    std::thread::spawn(|| {
        listener.for_each(move |event| {
            match event {
            NodeEvent::Network(event) => {
                match event {
                NetEvent::Connected(_, _) => unreachable!(), // Used for explicit connections.
                NetEvent::Accepted(endpoint, _) => {
                    println!("Client connected!");
                    let mut server = server.lock().unwrap();
                    
                    server.game.players.push(Player::default());
                    let players_cnt = server.game.players.len();

                    server.clients.insert(endpoint, players_cnt - 1);
                },
                NetEvent::Message(endpoint, data) => {
                    let msg: ClientMsg = bincode::deserialize(data).unwrap();
                    match msg {
                    ClientMsg::Id(name) => {
                        let mut server = server.lock().unwrap();
                        let player_id = *server.clients.get(&endpoint).unwrap();
                        server.game.players[player_id].name = name.clone();

                        server.update_players();
                    },
                    _ => {
                        todo!()
                    },
                    }
                },
                NetEvent::Disconnected(endpoint) => {
                    let mut server = server.lock().unwrap();
                    if server.started {
                        let clients = &server.clients;
                        println!("Client {} disconnected! End of session :/", server.game.players[*clients.get(&endpoint).unwrap()].name);
                        server.handler.as_ref().unwrap().stop();
                    } else {
                        let player_id = *server.clients.get(&endpoint).unwrap();
                        let name = &server.game.players[player_id].name;
                        println!("Player {} disconnected!", name);
                        
                        server.game.players.remove(player_id);
                        server.update_players();
                    }
                }, //Tcp or Ws
                }
            },
            NodeEvent::Signal(signal) => {
                match signal {
                Signal::SendStart => {
                    server.lock().unwrap().started = true;
                }
                }
            },
            };
        });
    });

    println!("Server started!");
}