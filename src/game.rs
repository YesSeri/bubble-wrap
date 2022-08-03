use crate::bubble::Bubble;
use crate::geometry::GravSpeed;
use crate::player::Player;
use crate::{
    wasm4::{self, *},
    Point,
};
#[derive(Debug)]
pub struct Game {
    frame_count: u32,
    bubbles: Vec<Bubble>,
    player: Player,
    pub game_over: bool,
}

impl Game {
    pub const fn new() -> Self {
        Game {
            frame_count: 0,
            // bubbles: SmallVec::<[Bubble; 4]>::new(),
            bubbles: Vec::new(),
            player: Player::new(),
            game_over: false,
        }
    }
    fn next_frame(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
    }
    pub fn init(&mut self) {
        // self.bubbles.push(Bubble {
        //     point: Point {
        //         x: 45,
        //         y: 50,
        //         level: true,
        //     },
        //     diameter: 8,
        //     speed: GravSpeed::new(1),
        //     i: 0,
        //     ticker: 0,
        // });
        self.bubbles.push(Bubble {
            point: Point {
                x: 15,
                y: 160 - 64,
                level: false,
            },
            diameter: 16,
            speed: GravSpeed::new(0),
            i: 0,
            ticker: 0,
        });
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };

        self.frame_count += 1;
        if self.frame_count % 2 == 0 {
            self.player.update();

            self.check_projectile_hits();
            for bubble in self.bubbles.iter_mut() {
                bubble.update();
            }
        }
        self.game_over = self.check_player_hit();
    }
    pub fn draw(&self) {
        line(0, 80, 160, 80);

        for bubble in self.bubbles.iter() {
            bubble.draw();
        }
        self.player.draw();
    }
    // If the projectile hits, both the projectile and the bubble dies.
    fn check_projectile_hits(&mut self) {
        let mut remove: Option<usize> = None;
        if let Some(projectile) = &mut self.player.projectile {
            for (i, bubble) in self.bubbles.iter().enumerate() {
                let Point {
                    x: x0,
                    y: y0,
                    level: l0,
                } = bubble.point;
                let r0 = bubble.diameter;
                let Point {
                    x: x1,
                    y: y1,
                    level: l1,
                } = projectile.end;

                let s = format!("{:?}", (l0, l1));
                if l0 == l1 {
                    if x1 >= x0 && x1 <= (x0 + r0) {
                        if y1 < (y0 + r0) {
                            remove = Some(i);
                            break;
                        }
                    }
                }
            }
        }
        if let Some(r) = remove {
            trace("PROJECTILE HIT BUBBLE");
            self.player.projectile = None;
            self.bubbles.remove(r);
        }
    }
    fn check_player_hit(&mut self) -> bool {
        #[derive(Debug)]
        struct Rect {
            x0: u8,
            x1: u8,
            y0: u8,
            y1: u8,
        }
        let p_rect = Rect {
            x0: self.player.point.x,
            x1: self.player.point.x + Player::PLAYER_WIDTH as u8,
            y0: self.player.point.y - Player::PLAYER_HEIGHT as u8,
            y1: self.player.point.y,
        };
        for bubble in self.bubbles.iter() {
            if bubble.point.level != self.player.point.level {
                continue;
            }
            let b_rect = Rect {
                x0: bubble.point.x,
                x1: bubble.point.x + bubble.diameter,
                y0: bubble.point.y,
                y1: bubble.point.y + bubble.diameter,
            };
            if b_rect.x0 < p_rect.x1 && b_rect.x1 > p_rect.x0 {
                // trace("X TOUCH");
            }
            if b_rect.y1 > p_rect.y0 {}
        }
        return false;
    }
}
