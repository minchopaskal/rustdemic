use std::{net::SocketAddr, sync::{Arc, Mutex}};

use egui::{Layout, RichText, Color32};
use egui_miniquad as emq;
use miniquad as mq;

use crate::{multiplayer::{client::{Client, start_client}, server::{Server, start_server}}, game::Game};

#[derive(PartialEq)]
enum PlayerType {
    None,
    Guest,
    Host,
}

enum Stage {
    MainScreen,
    Lobby,
    Game(PlayerType),
}

struct AppUi {
    stage: Stage,
    name: String,
    host: String,
    port: String,
    kind: PlayerType,
    connected: bool,

    connect_fail: bool,
}

pub struct App {
    egui_mq: emq::EguiMq,
    client: Arc<Mutex<Client>>,
    server: Arc<Mutex<Server>>,
    ui: AppUi,
}

impl App {
    pub fn new(mq_ctx: &mut mq::Context) -> Self {
        Self {
            egui_mq: emq::EguiMq::new(mq_ctx),
            client: Default::default(),
            server: Default::default(),
            ui: AppUi {
                stage: Stage::MainScreen,
                name: String::default(),
                host: "127.0.0.1".to_string(),
                port: "6969".to_string(),
                kind: PlayerType::None,
                connected: false,
                connect_fail: false,
            },
        }
    }
}

impl mq::EventHandler for App {
    fn update(&mut self, _: &mut mq::Context) { }

    fn draw(&mut self, mq_ctx: &mut mq::Context) {
        mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
        mq_ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        mq_ctx.end_render_pass();

        // Run the UI code:
        self.egui_mq.run(mq_ctx, |_mq_ctx, egui_ctx| {
            match &self.ui.stage {
            Stage::MainScreen => {
                egui::CentralPanel::default().show(egui_ctx, |_| { // panel
                    egui::Area::new("main")
                        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                        .show(egui_ctx, |ui| { // area
                    
                        ui.horizontal(|ui| {
                            ui.label("Name:");
                            ui.text_edit_singleline(&mut self.ui.name);
                        });
        
                        if ui.button("Enter").clicked() {
                            self.ui.stage = Stage::Lobby;
                        }
                    }); // area
                }); // panel
            }
            Stage::Lobby => {
                egui::CentralPanel::default().show(egui_ctx, |_| { // panel
                    egui::Area::new("lobby")
                        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                        .show(egui_ctx, |ui| { // area
                        match self.ui.kind {
                        PlayerType::Guest => {
                            if self.ui.connect_fail {
                                ui.label(RichText::new("Failed to connect!").color(Color32::RED));
                            }

                            if self.ui.connected {
                                ui.label(RichText::new("Waiting for host to start the game...").color(Color32::RED));

                                ui.label("Connected players:");
                                for p in self.client.lock().unwrap().game.players.iter() {
                                    ui.label(format!("{}", p.name));
                                }
                                return;
                            }
        
                            ui.horizontal(|ui| {
                                ui.label("Host addr:");
                                ui.text_edit_singleline(&mut self.ui.host);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Port:");
                                ui.text_edit_singleline(&mut self.ui.port);
                            });

                            if ui.button("Connect").clicked() {
                                let addr = format!("{}:{}", self.ui.host, self.ui.port);
                                println!("Client addr: {addr}");
                                if let Some(client) = Client::new(&self.ui.name, &addr, false) {
                                    self.client = Arc::new(Mutex::new(client));
                                    start_client(self.client.clone());
                                    self.ui.connected = true;
                                } else {
                                    self.ui.connect_fail = true;
                                }
                            }
                        },
                        PlayerType::Host => {
                            if self.ui.connect_fail {
                                ui.label(RichText::new("Failed to connect!").color(Color32::RED));
                            }

                            if self.ui.connected {
                                if ui.button("Start Game").clicked() {

                                    self.ui.stage = Stage::Game(PlayerType::Host);
                                }
        
                                ui.label("Connected players:");
                                for p in self.server.lock().unwrap().game.players.iter() {
                                    ui.label(format!("{}", p.name));
                                }
                                return;
                            }
        
                            ui.horizontal(|ui| {
                                ui.label("Port:");
                                ui.text_edit_singleline(&mut self.ui.port);
                            });
                            if ui.button("Start Server").clicked() {
                                if let Some(server) = Server::new(&self.ui.port) {
                                    self.server = Arc::new(Mutex::new(server));
                                    start_server(self.server.clone());

                                    println!("Server started!");

                                    self.ui.connected = true;

                                    let mut client_addr = "127.0.0.1:".to_string();
                                    client_addr.push_str(&self.ui.port);
                                    if let Some(client) = Client::new(&self.ui.name, &client_addr, false) {
                                        println!("Starting host-client...");
                                        self.client = Arc::new(Mutex::new(client));
                                        start_client(self.client.clone());
                                        self.ui.connected = true;
                                    } else {
                                        self.server.lock().unwrap().stop();
                                        self.ui.connect_fail = true;
                                    }
                                } else {
                                    self.ui.connect_fail = true;
                                }
                            }
                        },
                        PlayerType::None => {
                            if ui.button("Host").clicked() {
                                self.ui.kind = PlayerType::Host;
                            }
        
                            if ui.button("Guest").clicked() {
                                self.ui.kind = PlayerType::Guest;
                            }
                        },
                        }
                    }); // area
                });
            }
            Stage::Game(kind) => {
                match self.ui.kind {
                    PlayerType::Guest => {
                        egui::Window::new("Game Guest").show(egui_ctx, |ui| {
                            if ui.button("play").clicked() {
                                println!("Playing...");
                            }
                        });
                    },
                    PlayerType::Host => {
                        egui::Window::new("Game Host").show(egui_ctx, |ui| {
                            if ui.button("play").clicked() {
                                println!("Playing...");
                            }
                        });
                    },
                    _ => { panic!("Something's not right!"); },
                };
            },
            }
        });

        self.egui_mq.draw(mq_ctx);

        mq_ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, _: &mut mq::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, _: &mut mq::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut mq::Context,
        character: char,
        _keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}