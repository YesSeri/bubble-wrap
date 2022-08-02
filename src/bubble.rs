use crate::geometry::{Point, Speed};
use crate::wasm4::oval;
use crate::wasm4::trace;
pub struct Bubble {
    pub point: Point,
    pub radius: u8,
    pub speed: Speed,
    pub i: i32,
}

const GRAVITY: i8 = 2;
impl Bubble {
    pub fn draw(&self) {
        oval(
            self.point.x.into(),
            self.point.y.into(),
            self.radius.into(),
            self.radius.into(),
        )
    }
    pub fn update(&mut self) {
        self.speed.y += GRAVITY;
        // self.point.x += self.speed.x as u8;

        // let s = format!("{} {}", self.speed.y, self.point.y);
        // trace(s);
        self.point.y = (self.point.y as i16 + self.speed.y as i16) as u8;
        if self.point.y >= 160 {
            self.speed.y = -self.speed.y - 2
        }

        self.point.x = (self.point.x as i16 + self.speed.x as i16) as u8;

        if self.point.x >= 155 {
            self.speed.x = -self.speed.x - 1
        }

        if self.point.x <= 5 {
            self.speed.x = -self.speed.x + 1
        }
    }
}
