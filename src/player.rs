use crate::geometry::{Moveable, Point, Speed};
use crate::palette::set_draw_colors;
use crate::wasm4;
use crate::wasm4::{blit, line, trace};

// Small
// const PLAYER_WIDTH: u32 = 4;
// const PLAYER_HEIGHT: u32 = 8;
// const PLAYER_FLAGS: u32 = 1; // BLIT_2BPP
// const PLAYER: [u8; 8] = [0xc0, 0x05, 0x15, 0x19, 0xd5, 0xea, 0xea, 0xcc];
// Big
// const PLAYER_WIDTH: u32 = 8;
// const PLAYER_HEIGHT: u32 = 16;
// const PLAYER_FLAGS: u32 = 1; // BLIT_2BPP
// #[rustfmt::skip]
// const PLAYER: [u8; 32] = [ 0xf0,0x03,0xc0,0x00,0x01,0x55,0x06,0x96,0x16,0x96,0x15,0x55,0x15,0x55,0x2a,0x5a,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xeb,0xfa,0xdf,0xf7,0xc3,0xf0 ];
#[derive(Debug)]
pub struct Player {
    pub point: Point,
    pub projectile: Option<Projectile>,
}

impl Player {
    // Small
    // pub const PLAYER_WIDTH: u32 = 4;
    // pub const PLAYER_HEIGHT: u32 = 8;

    // const PLAYER_FLAGS: u32 = 1; // BLIT_2BPP
    // const PLAYER: [u8; 8] = [0xc0, 0x05, 0x15, 0x19, 0xd5, 0xea, 0xea, 0xcc];

    // Big
    pub const PLAYER_WIDTH: u32 = 8;
    pub const PLAYER_HEIGHT: u32 = 16;

    const PLAYER_FLAGS: u32 = 1; // BLIT_2BPP
    #[rustfmt::skip]
    const PLAYER: [u8; 32] = [ 0xf0,0x03,0xc0,0x00,0x01,0x55,0x06,0x96,0x16,0x96,0x15,0x55,0x15,0x55,0x2a,0x5a,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xeb,0xfa,0xdf,0xf7,0xc3,0xf0 ];
    pub const fn new() -> Self {
        Self {
            point: Point::new(5, 160),
            projectile: None,
        }
    }
    pub fn draw(&self) {
        set_draw_colors(0x0423);
        blit(
            &Player::PLAYER,
            self.point.x as i32,
            self.point.y as i32 - Player::PLAYER_HEIGHT as i32,
            Player::PLAYER_WIDTH,
            Player::PLAYER_HEIGHT,
            Player::PLAYER_FLAGS,
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
    pub fn shoot(&mut self) {
        if self.projectile.is_none() {
            self.projectile = Projectile::new(self.mid_point());
        }
    }
    fn mid_point(&self) -> Point {
        let (_, x) = Point::wrap_add(self.point.x, (Player::PLAYER_WIDTH / 2) as u8);
        Point::new(x, self.point.y)
    }
}
impl Moveable for Player {
    fn move_right(&mut self) {
        let x = self.point.x + 2;

        self.point.x = {
            if x < 160 {
                x
            } else {
                self.point.switch_level();
                0
            }
        }
    }
    fn move_left(&mut self) {
        let x = self.point.x.checked_sub(2);
        self.point.x = x.unwrap_or_else(|| {
            self.point.switch_level();
            159
        })
    }
}
#[derive(Debug)]
pub struct Projectile {
    pub start: Point,
    pub end: Point,
    pub speed: Speed,
}

impl Projectile {
    fn new(p: Point) -> Option<Self> {
        Some(Self {
            end: Point::new(p.x, p.y - 16),
            start: p,
            speed: Speed {
                x: 0,
                y: 3,
                down: false,
                right: true,
            },
        })
    }
    // Integer overflow gets checked in projectile_move_up
    fn is_finished(&self) -> bool {
        let dist = self.start.y - self.end.y;
        if dist == 80 {
            return true;
        }
        false
    }
    pub fn update(&mut self) {
        self.move_up();
    }
    pub fn draw(&self) {
        // let height: i32 = PLAYER_HEIGHT.try_into().unwrap_or(0);
        line(
            self.start.x.into(),
            self.start.y.into(),
            self.end.x.into(),
            self.end.y.into(),
        );
    }
}

impl Moveable for Projectile {
    fn move_up(&mut self) {
        if !self.speed.down {
            // if end.y - speed.y < 0 we get integer overflow, end.y = 254, or crash. That means it is ugly in release mode and crashes in debug mode.
            self.end.y = self.end.y.saturating_sub(self.speed.y);

            // To make sure the shoot doesnt go through the roof.
            if !self.end.level && self.end.y < 80 {
                self.end.y = 80;
            }
        }
    }
}