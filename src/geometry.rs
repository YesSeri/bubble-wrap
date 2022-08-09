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
        let level = y <= 80;
        Point { x, y, level }
    }
    pub fn switch_level(&mut self) {
        if self.level {
            self.y += 78;
        } else {
            self.y -= 78;
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
    pub x: u8,
    pub right: bool,
    pub y: i8,
}

const GRAVITY: i8 = 1;
impl GravSpeed {
    pub const fn new(x: u8, right: bool) -> Self {
        Self { x, y: 0, right }
    }
    pub fn add_gravity(&mut self) {
        self.y += GRAVITY;
    }
}
