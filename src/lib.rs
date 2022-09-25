// #![allow(unused_imports)]
#![allow(clippy::collapsible_if)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![no_std]
// #![no_main]
// #[panic_handler]
// fn handle_panic(_info: &core::panic::PanicInfo) -> ! {
//     trace(_info);
//     loop {}
// }
#[cfg(feature = "buddy-alloc")]
mod alloc;
mod background;
mod bubble;
mod game;
mod geometry;
mod music;
mod palette;
mod pixel_manip;
mod player;
mod wasm4;

use background::Background;
pub use bubble::Bubble;
pub use game::Game;
pub use geometry::{Point, Speed};
use music::update_music;
use palette::set_draw_colors;
pub use player::Player;
use wasm4::*;

#[no_mangle]
fn update() {
    unsafe {
        update_music();
        Background::draw();
        GAME.draw();
        match GAME.game_state {
            game::GameState::Playing => {
                GAME.update();
            }
            game::GameState::GameOver => {
                text("GAME OVER!", 40, 50);
            }
            game::GameState::Victory => {
                if GAME.difficulty >= 7 {
                    set_draw_colors(0x24);
                    rect(0, 44, 160, 20);
                    rect(0, 94, 160, 20);
                    set_draw_colors(0x14);
                    text("YOU HAVE WON!!!", 20, 50);
                    text("CONGRATULATIONS!", 18, 100);
                    set_draw_colors(0x34);
                } else {
                    set_draw_colors(0x44);
                    rect(0, 44, 160, 20);
                    rect(0, 94, 160, 20);
                    set_draw_colors(0x41);
                    text("LEVEL COMPLETED!", 20, 50);
                    text("PRESS BUTTON 1", 32, 100);
                    set_draw_colors(0x34);
                    GAME.start_next_level();
                }
            }
        }
    }

    set_draw_colors(0x04);
    text("LEVEL:", 8, 6);
    let difficulty = unsafe { GAME.difficulty };

    for x in 0..difficulty {
        let offset_x = 4 * (x % 5) as i32;
        let offset_y = 8 * (x / 5) as i32;
        rect(56 + offset_x, 6 + offset_y, 3, 7);
    }
    // show_color_palette();
}

fn show_color_palette() {
    for color in 1..=4 {
        let color = color;
        set_draw_colors(color.try_into().unwrap());
        rect(color * 20, 0, 20, 20);
        if color == 1 {
            set_draw_colors(3);
        } else {
            set_draw_colors((color - 1).try_into().unwrap());
        }
        text(color.to_string(), color * 20, 0);
        set_draw_colors(0x1234);
    }
    vline(19, 0, 21);
    hline(20, 20, 40);

    set_draw_colors(0x1232);
    hline(60, 20, 40);
    vline(100, 0, 21);
}
// static mut f: u32 = 0;
// static mut t: u32 = 0;
#[no_mangle]
fn start() {
    // palette::set_palette([0xdbd7d3, 0xe5b083, 0x426e5d, 0x20283d]);
    // palette::set_palette([0xffffff, 0xaaaaaa, 0x666666, 0x111111]);
    palette::set_palette([0xfff6d3, 0xf9a875, 0xeb6b6f, 0x7c3f58]);
    set_draw_colors(0x1234);
    unsafe {
        GAME.setup_level();
    }
}

static mut GAME: Game = Game::new();
