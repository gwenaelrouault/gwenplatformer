#![deny(clippy::all)]
#![forbid(unsafe_code)]

use g2d_engine::G2dEngine;
use pixels::Error;

mod game_gui;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const BOX_SIZE: i16 = 64;

fn main() -> Result<(), Error> {
    // init logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // load background
    let background = image::open("./resources/background.png")
        .unwrap()
        .to_rgba8();

    // init GUI
    let gui = Box::new(game_gui::GameGui::new());

    // init engine
    let mut engine = G2dEngine::new(WIDTH, HEIGHT, background);
    engine.run(gui)
}
