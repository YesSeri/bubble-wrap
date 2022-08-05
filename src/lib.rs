// #![allow(unused_imports)]
// #![allow(clippy::collapsible_if)]
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
mod bubble;
mod game;
mod geometry;
mod palette;
mod player;
mod wasm4;
pub use bubble::Bubble;
pub use game::Game;
pub use geometry::{Point, Speed};
use palette::set_draw_colors;
pub use player::Player;
use wasm4::*;

#[rustfmt::skip]
const wall: [u8; 8] = [    
	0b00000000,
    0b00000010,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000100,
    0b01000000,
    0b00000000,
];
fn draw_wall() {
    set_draw_colors(0x21);
    for y in 0..20 {
        for x in 0..20 {
            if (y - x) % 3 == 0 && x % 7 == 0 || y % 5 == 0 && (x - y) % 3 == 0 {
                blit(&wall, x * 8, y * 8, 8, 8, BLIT_1BPP);
            } else if (y * 7 / 5 - x / 2) % 2 == 0 {
                blit(&wall, x * 8, y * 8, 8, 8, BLIT_1BPP | BLIT_FLIP_Y);
            } else if y % 3 == 0 && x % 4 == 0 {
                // blit(&wall, x * 8, y * 8, 8, 8, BLIT_1BPP | BLIT_FLIP_X);
            }
        }
    }

    set_draw_colors(0x1213);
    line(0, 80, 160, 80);

    set_draw_colors(0x1234);
}

#[no_mangle]
fn update() {
    unsafe {
        draw_wall();
        GAME.draw();
        match GAME.game_state {
            game::GameState::Playing => {
                GAME.update();
            }
            game::GameState::GameOver => {
                text("GAME OVER!", 40, 50);
            }
            game::GameState::Victory => {
                set_draw_colors(0x44);
                rect(0, 44, 160, 20);
                set_draw_colors(0x41);
                text("YOU HAVE WON!", 30, 50);
                set_draw_colors(0x34)
            }
        }
        // if GAME.game_over {
        //     GAME.draw();
        //     text("GAME OVER!", 40, 50);
        //     return;
        // }
    }
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
#[no_mangle]
fn start() {
    palette::set_palette([0xdbd7d3, 0xe5b083, 0x426e5d, 0x20283d]);
    set_draw_colors(0x1234);
    unsafe {
        GAME.init();
    }
}

static mut GAME: Game = Game::new();
