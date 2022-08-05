#[derive(Debug, Clone)]
pub struct OutOfBoundsError;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u8,
    pub y: u8,
    pub level: bool,
}
impl Point {
    pub const fn new(x: u8, y: u8) -> Self {
        let level = y < 80;
        Point { x, y, level }
    }
    // pub fn right(&mut self) {
    //     let (wrapped, x) = Point::wrap_add(self.x, 1);
    //     if wrapped {
    //         self.switch_level()
    //     }
    //     self.x = x;
    // }
    // pub fn left(&mut self) {
    //     let (wrapped, x) = Point::wrap_sub(self.x, 1);
    //     if wrapped {
    //         self.switch_level()
    //     }
    //     self.x = x;
    // }
    // pub fn up(&mut self, speed: i8) {
    //     self.y = (self.y as i16 - speed as i16) as u8;
    //     // If y is bigger than screen that can crash game. it can lead to two u8s: 13 - 255.
    // }
    // pub fn wrap_add(x: u8, y: u8) -> (bool, u8) {
    //     let xpr = x + y;
    //     let wrapped = xpr / 160;
    //     (wrapped != 0, xpr % 160)
    // }
    // pub fn wrap_sub(x: u8, y: u8) -> (bool, u8) {
    //     let z = x.checked_sub(y);
    //     (z.is_none(), z.unwrap_or(159))
    // }

    pub fn switch_level(&mut self) {
        if self.level {
            self.y += 80;
        } else {
            self.y -= 80;
        }
        self.level = !self.level;
    }
    pub fn get_y(&self) -> u8 {
        match &self.level {
            false => self.y,
            _ => self.y - 80,
        }
    }
    // pub fn change_pos(&mut self, speed: Speed){

    // }
}

#[derive(Debug)]
pub struct Speed {
    pub x: u8,
    pub y: u8,
    pub down: bool,
    pub right: bool,
}

impl Speed {
    pub const fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            down: false,
            right: false,
        }
    }
    // pub fn should_turn(i: i8) -> bool {
    //     match i {
    //         4 | -4 => true,
    //         _ => false,
    //     }
    // }
}

pub trait Moveable {
    fn move_right(&mut self) {}
    fn move_left(&mut self) {}
    fn move_up(&mut self) {}
    fn move_down(&mut self) {}
    // fn check_y_overflow(&mut self, ){

    // }

    // fn check_x_overflow(&mut self, ){

    // }
}

#[derive(Debug)]
pub struct GravSpeed {
    pub x: i8,
    pub y: i8,
}

const GRAVITY: i8 = 1;
impl GravSpeed {
    pub const fn new(x: i8) -> Self {
        Self { x, y: 0 }
    }
    pub fn add_gravity(&mut self) {
        self.y += GRAVITY;
    }
}
