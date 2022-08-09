#![allow(unused_imports)]
#![allow(clippy::collapsible_if)]
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
mod music;
mod palette;
mod player;
mod wasm4;

pub use bubble::Bubble;
pub use game::Game;
pub use geometry::{Point, Speed};
use music::update_music;
use palette::set_draw_colors;
pub use player::Player;
use wasm4::*;

#[rustfmt::skip]
const WALL: [u8; 8] = [    
	0b00000000,
    0b00000010,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000100,
    0b01000000,
    0b00000000,
];
#[rustfmt::skip]
const SIDE: [u8; 32] = [ 0x05, 0x6a, 0x01, 0x5a, 0x01, 0x6a, 0x01, 0xaa, 0x05, 0x6a, 0x15, 0x5a, 0x05, 0x5a, 0x05, 0x6a, 0x01, 0x6a, 0x01, 0x6a, 0x05, 0x6a, 0x05, 0xaa, 0x05, 0xaa, 0x05, 0x6a, 0x01, 0x6a, 0x05, 0xaa, ];
#[rustfmt::skip]
const CORNER: [u8; 64] = [ 0x00,0x00,0x00,0x00,0x00,0x10,0x01,0x56,0x01,0x55,0x55,0xaa,0x01,0x55,0x9a,0xbf,0x05,0x6a,0xab,0xff,0x15,0x5a,0xaf,0xff,0x05,0x5a,0xbf,0xff,0x05,0x6a,0xff,0xff,0x01,0x6a,0xff,0xff,0x01,0x6a,0xff,0xff,0x05,0x6a,0xff,0xff,0x05,0xaa,0xff,0xff,0x05,0xaa,0xff,0xff,0x05,0x6a,0xff,0xff,0x01,0x6a,0xff,0xff,0x05,0xaa,0xff,0xff ];
fn draw_wall() {
    set_draw_colors(0x13);
    for y in 0..20 {
        for x in 0..20 {
            blit(&WALL, x * 8, y * 8, 8, 8, BLIT_1BPP);
        }
    }

    set_draw_colors(0x0234);
    for i in 1..9 {
        blit(&SIDE, 0, i * 16, 8, 16, BLIT_2BPP);
        blit(&SIDE, 152, i * 16, 8, 16, BLIT_2BPP | BLIT_FLIP_X);
    }

    line(0, 80, 160, 80);
    blit(&CORNER, 0, 0, 16, 16, BLIT_2BPP);
    blit(&CORNER, 0, 80, 16, 16, BLIT_2BPP);

    blit(&CORNER, 144, 0, 16, 16, BLIT_2BPP | BLIT_FLIP_X);
    blit(&CORNER, 144, 80, 16, 16, BLIT_2BPP | BLIT_FLIP_X);

    blit(&CORNER, 0, 144, 16, 16, BLIT_2BPP | BLIT_FLIP_Y);
    blit(&CORNER, 0, 65, 16, 16, BLIT_2BPP | BLIT_FLIP_Y);
    blit(
        &CORNER,
        144,
        144,
        16,
        16,
        BLIT_2BPP | BLIT_FLIP_X | BLIT_FLIP_Y,
    );
    blit(
        &CORNER,
        144,
        65,
        16,
        16,
        BLIT_2BPP | BLIT_FLIP_X | BLIT_FLIP_Y,
    );

    set_draw_colors(0x2);
    line(16, 158, 144, 158);
    line(16, 1, 144, 1);
    set_draw_colors(0x4);
    line(16, 159, 144, 159);
    line(16, 0, 144, 0);
    set_draw_colors(0x4);

    set_draw_colors(0x1234);
}

#[no_mangle]
fn update() {
    unsafe {
        update_music();
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
static mut f: u32 = 0;
static mut t: u32 = 0;
#[no_mangle]
fn start() {
    // palette::set_palette([0xdbd7d3, 0xe5b083, 0x426e5d, 0x20283d]);
    // palette::set_palette([0xffffff, 0xaaaaaa, 0x666666, 0x111111]);
    palette::set_palette([0xfff6d3, 0xf9a875, 0xeb6b6f, 0x7c3f58]);
    set_draw_colors(0x1234);
    unsafe {
        GAME.init();
    }
}

static mut GAME: Game = Game::new();
