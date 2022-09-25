use crate::{palette::set_draw_colors, wasm4::*};
pub struct Background;

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

impl Background {
    fn draw_side() {
        for i in 1..9 {
            blit(&SIDE, 0, i * 16, 8, 16, BLIT_2BPP);
            blit(&SIDE, 152, i * 16, 8, 16, BLIT_2BPP | BLIT_FLIP_X);
        }
    }
    fn draw_corners() {
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
    }
    fn draw_wall() {
        for y in 0..20 {
            for x in 0..20 {
                blit(&WALL, x * 8, y * 8, 8, 8, BLIT_1BPP);
            }
        }
    }

    pub fn draw() {
        set_draw_colors(0x13);
        Background::draw_wall();

        set_draw_colors(0x0234);
        Background::draw_side();

        line(0, 80, 160, 80);
        Background::draw_corners();
        set_draw_colors(0x2);
        line(16, 158, 144, 158);
        line(16, 1, 144, 1);
        set_draw_colors(0x4);
        line(16, 159, 144, 159);
        line(16, 0, 144, 0);
        set_draw_colors(0x4);

        set_draw_colors(0x1234);
    }
}
