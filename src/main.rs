mod app;
mod game;
mod util;
mod multiplayer;

use miniquad as mq;
use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let conf = mq::conf::Conf {
        window_title: "Rustdemic".to_string(),
        high_dpi: true,
        window_resizable: true,
        icon: Some(mq::conf::Icon {
            small: [0; 16*16*4],
            medium: [0; 32*32*4],
            big: [0; 64*64*4],
        }),
        ..Default::default()
    };

    mq::start(conf, |mq_ctx| Box::new(App::new(mq_ctx)));

    Ok(())
}
