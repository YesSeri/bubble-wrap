use crate::geometry::{Point, Speed};
use crate::palette::set_draw_colors;
use crate::wasm4;
use crate::wasm4::{blit, line, trace};

const PLAYER_WIDTH: u32 = 8;
const PLAYER_HEIGHT: u32 = 16;
const PLAYER_FLAGS: u32 = 1; // BLIT_2BPP
#[rustfmt::skip]
const PLAYER: [u8; 32] = [ 0xf0,0x03,0xc0,0x00,0x01,0x55,0x06,0x96,0x16,0x96,0x15,0x55,0x15,0x55,0x2a,0x5a,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xeb,0xfa,0xdf,0xf7,0xc3,0xf0 ];
#[derive(Debug)]
pub struct Player {
    pub point: Point,
    pub projectile: Option<Projectile>,
}

impl Player {
    pub const fn new() -> Self {
        Self {
            point: Point::new(5, 160, false),
            projectile: None,
        }
    }
    pub fn draw(&self) {
        set_draw_colors(0x0423);
        blit(
            &PLAYER,
            self.point.x as i32,
            self.point.y as i32 - PLAYER_HEIGHT as i32,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            PLAYER_FLAGS,
        );

        if let Some(p) = &self.projectile {
            p.draw();
        }
    }
    pub fn update(&mut self) {
        // let s = format!("{:?}", &self);
        // trace(s);
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        if gamepad & wasm4::BUTTON_RIGHT != 0 {
            self.move_right();
        }
        if gamepad & wasm4::BUTTON_LEFT != 0 {
            self.move_left();
        }
        if gamepad & wasm4::BUTTON_UP != 0 {
            self.shoot();
        }

        if let Some(p) = &self.projectile {
            if p.is_finished() {
                self.projectile = None;
            }
        }

        if let Some(p) = &mut self.projectile {
            p.update();
        }
    }
    pub fn move_right(&mut self) {
        self.point.right();
    }
    pub fn move_left(&mut self) {
        self.point.left()
    }
    pub fn shoot(&mut self) {
        if self.projectile.is_none() {
            self.projectile = Projectile::new(self.mid_point());
        }
    }
    fn mid_point(&self) -> Point {
        let (_, x) = Point::wrap_add(self.point.x, (PLAYER_WIDTH / 2) as u8);
        Point::new(x, self.point.y, self.point.level)
    }
}

#[derive(Debug)]
pub struct Projectile {
    start: Point,
    end: Point,
    speed: Speed,
}

impl Projectile {
    fn new(p: Point) -> Option<Self> {
        Some(Self {
            end: Point::new(p.x, p.y - 1, p.level),
            start: p,
            speed: Speed { x: 0, y: -1 },
        })
    }
    fn is_finished(&self) -> bool {
        if self.end.y <= 80 {
            return true;
        }
        false
    }
    pub fn update(&mut self) {
        self.end.up(2);
    }
    pub fn draw(&self) {
        line(
            self.start.x.into(),
            self.start.get_y().into(),
            self.end.x.into(),
            self.end.get_y().into(),
        );
    }
}
