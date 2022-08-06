use crate::geometry::{GravSpeed, Moveable, Point};
use crate::palette::set_draw_colors;
use crate::wasm4::{oval, rect};

#[derive(Debug)]
pub struct Bubble {
    pub point: Point,
    pub diameter: u8,
    pub speed: GravSpeed,
    pub ticker: u8,
}

impl Bubble {
    pub fn new(point: Point, diameter: u8, speed: i8) -> Self {
        Self {
            point,
            diameter,
            speed: GravSpeed::new(speed),
            ticker: 0,
        }
    }
    pub fn draw(&self) {
        set_draw_colors(0x41);
        oval(
            self.point.x.into(),
            self.point.y.into(),
            self.diameter.into(),
            self.diameter.into(),
        );

        set_draw_colors(0x44);

        for i in 0..2 {
            rect(
                self.point.x as i32 + self.diameter as i32 / 2 + i,
                self.point.y as i32 + self.diameter as i32 / 2 + i,
                1,
                1,
            );
            rect(
                self.point.x as i32 + self.diameter as i32 / 2 - i,
                self.point.y as i32 + self.diameter as i32 / 2 - i,
                1,
                1,
            );
            rect(
                self.point.x as i32 + self.diameter as i32 / 2 + i,
                self.point.y as i32 + self.diameter as i32 / 2 - i,
                1,
                1,
            );
            rect(
                self.point.x as i32 + self.diameter as i32 / 2 - i,
                self.point.y as i32 + self.diameter as i32 / 2 + i,
                1,
                1,
            );
        }

        set_draw_colors(0x1234);
    }

    pub fn update(&mut self) {
        self.ticker += 1;

        if self.ticker % 3 != 0 && self.ticker % 7 != 0 {
            self.speed.add_gravity();
        }

        self.movement();
    }

    fn movement(&mut self) {
        self.point.y = (self.point.y as i16 + self.speed.y as i16) as u8;

        if self.point.level && (self.point.y > (80 - self.diameter)) {
            // This ensures an even bounce after first bounce.
            self.clamp_speed();
            self.point.y = 80 - self.diameter;
            self.speed.y = -self.speed.y;
        }

        if self.point.y > (160 - self.diameter) {
            self.clamp_speed();
            self.point.y = 160 - self.diameter;
            self.speed.y = -self.speed.y;
        }

        if self.speed.x > 0 {
            self.move_right();
        } else {
            self.move_left();
        }
    }
    fn clamp_speed(&mut self) {
        self.ticker = 0;
        if self.speed.y % 2 != 0 {
            self.speed.y -= 1;
        }
        let min_speed = 6;
        if self.speed.y < min_speed {
            self.speed.y = min_speed;
        }
    }
}

impl Moveable for Bubble {
    fn move_right(&mut self) {
        // let x = self.point.x + self.speed.x;
        let x = self.point.x + self.speed.x as u8;

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
        unsafe {
            let x = self
                .point
                .x
                .checked_sub(self.speed.x.try_into().unwrap_unchecked());
            self.point.x = x.unwrap_or_else(|| {
                self.point.switch_level();
                159
            })
        }
    }
}
