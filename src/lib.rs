#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
// #![no_std]
// #![no_main]
// #[panic_handler]
// fn handle_panic(_info: &core::panic::PanicInfo) -> ! {
//     trace(_info);
//     loop {}
// }

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod bubble;
mod game;
mod geometry;
mod palette;
mod player;
mod wasm4;
pub use bubble::Bubble;
pub use game::Game;
use geometry::GravSpeed;
pub use geometry::{Point, Speed};
use palette::set_draw_colors;
pub use player::Player;
use wasm4::*;

#[no_mangle]
fn update() {
    unsafe {
        GAME.draw();
        match GAME.game_state {
            game::GameState::Playing => {
                GAME.update();
            }
            game::GameState::GameOver => {
                text("GAME OVER!", 40, 50);
            }
            game::GameState::Victory => {
                text("YOU HAVE WON!", 30, 50);
            }
        }
        // if GAME.game_over {
        //     GAME.draw();
        //     text("GAME OVER!", 40, 50);
        //     return;
        // }
    }
}
#[no_mangle]
fn start() {
    palette::set_palette([0xfbf7f3, 0xe5b083, 0x426e5d, 0x20283d]);
    set_draw_colors(0x1234);
    unsafe {
        GAME.init();
    }
}

static mut GAME: Game = Game::new();
