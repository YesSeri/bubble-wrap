use crate::wasm4::*;
type PointResult = std::result::Result<(), OutOfBoundsError>;

#[derive(Debug, Clone)]
pub struct OutOfBoundsError;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u8,
    pub y: u8,
    pub level: bool,
}
impl Point {
    pub const fn new(x: u8, y: u8, level: bool) -> Self {
        Point { x, y, level }
    }
    pub fn right(&mut self) {
        let (wrapped, x) = Point::wrap_add(self.x, 1);
        if wrapped {
            self.switch_level()
        }
        self.x = x;
    }
    pub fn left(&mut self) {
        let (wrapped, x) = Point::wrap_sub(self.x, 1);
        if wrapped {
            self.switch_level()
        }
        self.x = x;
    }
    pub fn up(&mut self, speed: i8) {
        // let s = format!("{:?}", &self);
        // trace(s);
        self.y = (self.y as i16 - speed as i16) as u8;
        // If y is bigger than screen that can crash game. it can lead to two u8s: 13 - 255.
    }
    pub fn wrap_add(x: u8, y: u8) -> (bool, u8) {
        let xpr = x + y;
        let wrapped = xpr / 160;
        (wrapped != 0, xpr % 160)
    }
    pub fn wrap_sub(x: u8, y: u8) -> (bool, u8) {
        // let z = (x as i16 - y as i16) as u8;

        // let s = format!("{x} - {y} = {z}",);
        let z = x.checked_sub(y);
        (z.is_none(), z.unwrap_or(159))
    }

    pub fn switch_level(&mut self) {
        self.level = !self.level;
    }
    pub fn get_y(&self) -> u8 {
        match &self.level {
            false => self.y,
            _ => {
                let s = format!("{:?}", &self);
                trace(s);
                self.y - 80
            }
        }
    }
    // pub fn change_pos(&mut self, speed: Speed){

    // }
}

#[derive(Debug)]
pub struct Speed {
    pub x: i8,
    pub y: i8,
}

impl Speed {
    // pub fn should_turn(i: i8) -> bool {
    //     match i {
    //         4 | -4 => true,
    //         _ => false,
    //     }
    // }
}

trait Moveable {
    fn right(&self) -> u8;
    fn up(&self) -> u8;
    fn down(&self) -> u8;
}
