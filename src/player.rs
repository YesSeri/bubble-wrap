use crate::geometry::{Moveable, Point, Speed};
use crate::palette::set_draw_colors;
use crate::wasm4::{self};
use crate::wasm4::{blit, line};

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
    moving: bool,
    first_sprite: bool,
    limiter: u8,
    speed: Speed,
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
    #[rustfmt::skip]
    const PLAYER2: [u8; 32] =[ 0xf0,0x03,0xc0,0x00,0x01,0x55,0x06,0x96,0x16,0x96,0x15,0x55,0x15,0x55,0x2a,0x5a,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xea,0xaa,0xfa,0xeb,0xfd,0xdf,0xfc,0x03 ];
    pub const fn new() -> Self {
        Self {
            point: Point::new(15, 158),
            projectile: None,
            moving: false,
            first_sprite: false,
            limiter: 0,
            speed: Speed::new(),
        }
    }
    pub fn draw(&self) {
        set_draw_colors(0x0424);
        if self.moving && !self.first_sprite {
            blit(
                &Player::PLAYER2,
                self.point.x as i32,
                self.point.y as i32 - Player::PLAYER_HEIGHT as i32,
                Player::PLAYER_WIDTH,
                Player::PLAYER_HEIGHT,
                Player::PLAYER_FLAGS,
            );
        } else {
            blit(
                &Player::PLAYER,
                self.point.x as i32,
                self.point.y as i32 - Player::PLAYER_HEIGHT as i32,
                Player::PLAYER_WIDTH,
                Player::PLAYER_HEIGHT,
                Player::PLAYER_FLAGS,
            );
        }
        if let Some(p) = &self.projectile {
            p.draw();
        }
    }
    pub fn update(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        if gamepad & wasm4::BUTTON_RIGHT != 0 {
            self.limiter += 1;
            self.moving = true;
            self.move_right();
            if self.limiter == 5 {
                self.first_sprite = !self.first_sprite;
                self.limiter = 0;
            }
        } else if gamepad & wasm4::BUTTON_LEFT != 0 {
            self.limiter += 1;
            self.moving = true;
            self.move_left();
            if self.limiter == 5 {
                self.first_sprite = !self.first_sprite;
                self.limiter = 0;
            }
        } else {
            self.moving = false;
            self.first_sprite = true;
        }

        if gamepad & wasm4::BUTTON_UP != 0 {
            self.moving = true;
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
        let x_mid = self.point.x + Player::PLAYER_WIDTH as u8 / 2;
        if x_mid <= 160 {
            Point::new(x_mid, self.point.y)
        } else {
            Point::new(160, self.point.y)
        }
    }
}
impl Moveable for Player {
    fn move_right(&mut self) {
        self.speed.right = true;
        self.speed.x = 2;
        let x = self.point.x + self.speed.x;

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
        self.speed.right = false;
        self.speed.x = 2;
        let x = self.point.x.checked_sub(self.speed.x);
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
        if dist >= 78 {
            return true;
        }
        false
    }
    pub fn update(&mut self) {
        self.move_up();
    }
    pub fn draw(&self) {
        let x1 = self.start.x;
        // let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;
        // let y1 = self.start.y;
        let y1 = {
            let floor_height = {
                if self.start.level {
                    80
                } else {
                    160
                }
            };
            // Makes the projectile grow downwards as well as upwards, so it looks shoot from player head.
            let player_top_y = floor_height - Player::PLAYER_HEIGHT as u8;
            let diff = player_top_y - y2;

            if player_top_y + diff > floor_height {
                floor_height
            } else {
                player_top_y + diff
            }
            // let diff = self.start.y - Player::PLAYER_HEIGHT as u8- self.end.y;
        };
        line(x1.into(), y1.into(), x2.into(), y2.into());
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
