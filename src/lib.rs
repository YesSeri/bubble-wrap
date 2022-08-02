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
mod geometry;
mod palette;
mod player;
mod wasm4;
pub use bubble::Bubble;
pub use geometry::{Point, Speed};
pub use player::Player;
use wasm4::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

#[no_mangle]
fn update() {
    line(0, 80, 160, 80);
    let (player, frame_count, gamepad) =
        unsafe { (&mut GAME.player, &mut GAME.frame_count, *wasm4::GAMEPAD1) };

    // if *frame_count % 1 == 0 {
    //     // for bubble in GAME.bubbles.iter_mut() {
    //     //     bubble.update();
    //     // }
    // }

    *frame_count += 1;
    player.draw();
    player.update();

    // for bubble in GAME.bubbles.iter() {
    //     bubble.draw();
    // }
}
#[no_mangle]
fn start() {
    palette::set_palette([0xfbf7f3, 0xe5b083, 0x426e5d, 0x20283d]);
    unsafe {
        GAME.bubbles.push(Bubble {
            point: Point {
                x: 55,
                y: 55,
                level: false,
            },
            radius: 4,
            speed: Speed { x: 1, y: 0 },
            i: 0,
        })
    }
}

static mut GAME: Game = Game {
    frame_count: 0,
    // bubbles: SmallVec::<[Bubble; 4]>::new(),
    bubbles: Vec::new(),
    player: Player::new(),
};

struct Game {
    frame_count: u32,
    bubbles: Vec<Bubble>,
    player: Player,
}

impl Game {
    fn next_frame(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
    }
}
