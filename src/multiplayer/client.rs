use std::{io::Read, net::SocketAddr, collections::HashMap, thread::JoinHandle, sync::{Arc, Mutex}};

use message_io::{node::{NodeHandler, NodeListener, self, NodeEvent, NodeTask}, network::{Transport, Endpoint, NetEvent}};

use crate::game::{world::World, player::{Player, PlayerId}, Game};

use super::{ClientMsg, ServerMsg, MsgHash};

enum Signal {
    Greet,
}

#[derive(Default)]
pub struct Client {
    pub game: Game,

    pub player: Player,

    addr: String,
    endpoint: Option<Endpoint>,

    requests: HashMap<MsgHash, ClientMsg>,
    handler: Option<NodeHandler<Signal>>,
    listener: Option<NodeListener<Signal>>,

    node_task: Option<NodeTask>,

    connected: bool,
    is_server: bool,
}

#[derive(Debug)]
pub enum ClientError {
    NotConnected,
}

impl Client {
    pub fn new(name: &str, addr: &str, is_server: bool) -> Option<Self> {
        let (handler, listener) = node::split();

        Some(Self {
            game: Game::default(),
            player: Player {
                name: name.to_string(), 
                ..Default::default()
            },
            addr: addr.to_string(),
            endpoint: None,
            requests: HashMap::new(),
            handler: Some(handler),
            listener: Some(listener),
            node_task: None,
            connected: false,
            is_server,
        })
    }

    fn update_players(&mut self, v: Vec<Player>) {
        self.game.players = v;
    }

    pub fn players_map<F>(&self, f: F) where F: Fn(&Player) {
        for player in self.game.players.iter() {
            f(player);
        }
    }
}

pub fn start_client(client: Arc<Mutex<Client>>) {
    println!("Starting client...");

    let client_orig = client;
    let mut client = client_orig.lock().unwrap();
    let listener = client.listener.take().unwrap();

    let result = client.handler.as_ref().unwrap().network().connect(Transport::FramedTcp, &client.addr);

    if let Ok((endpoint, _)) = result {
        client.endpoint = Some(endpoint);

        let client = client_orig.clone();
        std::thread::spawn(|| {
            listener.for_each(move |event| {
                match event {
                NodeEvent::Network(net_event) => match net_event {
                    NetEvent::Connected(_endpoint, _ok) => {
                        client.lock().unwrap().handler.as_ref().unwrap().signals().send(Signal::Greet)
                    },
                    NetEvent::Accepted(_, _) => unreachable!(), // Only generated by listening
                    NetEvent::Message(_endpoint, data) => {
                        let mut client = client.lock().unwrap();
                        let msg: ServerMsg = bincode::deserialize(data).unwrap();
                        match msg {
                        ServerMsg::AddPlayer(players) => {
                            client.update_players(players);
                        },
                        _ => todo!(),
                        }
                    },
                    NetEvent::Disconnected(_endpoint) => {
                        client.lock().unwrap().handler.as_ref().unwrap().stop();
                        println!("Bye bye!");
                    },
                },
                NodeEvent::Signal(signal) => {
                    match signal {
                    Signal::Greet => {
                        let mut client = client.lock().unwrap();
                        client.connected = true;

                        let msg = ClientMsg::Id(client.player.name.clone());
                        let data = bincode::serialize(&msg).unwrap();
                        client.handler.as_ref().unwrap().network().send(client.endpoint.unwrap(), &data);
                    },
                    }
                },
                }
            });
        });
        println!("Client started!");
    }
}
