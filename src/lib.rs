#[macro_use]
extern crate lazy_static;

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

mod tile;
mod random;
mod game_stage;

use std::sync::{Mutex};

lazy_static! {
    static ref GAME_STAGE: Mutex<game_stage::GameStage> = Mutex::new(game_stage::GameStage::new());
}

#[no_mangle]
fn start() {
    match GAME_STAGE.lock() {
        Ok(mut game_stage) => {
            game_stage.start();
        }
        Err(err) => {
            panic!("Very bad situation during start! {}", err);
        }
    }
}

#[no_mangle]
fn update() {
    match GAME_STAGE.lock() {
        Ok(mut game_stage) => {
            game_stage.update();
            game_stage.render();
        }
        Err(err) => {
            panic!("Very bad situation during update! {}", err);
        }
    }
}