use crate::bubble::Bubble;
use crate::player::Player;
use crate::wasm4;
use crate::wasm4::trace;
use crate::Point;
// use arrayvec::ArrayVec as Vec;
use heapless::Vec;
#[derive(Debug)]
pub enum GameState {
    Playing,
    GameOver,
    Victory,
}
#[derive(Debug)]
pub struct Game {
    frame_count: u32,
    pub difficulty: u8,
    bubbles: Vec<Bubble, 16>,
    player: Player,
    pub game_state: GameState,
}

impl Game {
    pub const fn new() -> Self {
        Game {
            frame_count: 0,
            bubbles: Vec::<Bubble, 16>::new(),
            difficulty: 0,
            player: Player::new(),
            game_state: GameState::Playing,
        }
    }
    fn add_bubble(&mut self, x: u8, y: u8, diameter: u8, speed: u8, right: bool) {
        unsafe {
            self.bubbles
                .push(Bubble::new(Point::new(x, y), diameter, speed, right))
                .unwrap_unchecked();
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        self.player.update();

        self.check_projectile_hits();
        for bubble in self.bubbles.iter_mut() {
            bubble.update();
        }
        if self.check_player_hit() {
            self.game_state = GameState::GameOver;
        }
        if self.bubbles.is_empty() {
            self.game_state = GameState::Victory;
        }
    }
    pub fn draw(&self) {
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
            self.player.projectile = None;
            self.bubbles.swap_remove(r);
        }
    }
    fn check_player_hit(&mut self) -> bool {
        let p_x0 = self.player.point.x;
        let p_x1 = self.player.point.x + Player::PLAYER_WIDTH as u8;
        let p_y0 = self.player.point.y - Player::PLAYER_HEIGHT as u8;
        for bubble in self.bubbles.iter() {
            if bubble.point.level != self.player.point.level {
                continue;
            }

            let b_x0 = bubble.point.x;
            let b_x1 = bubble.point.x + bubble.diameter;
            let b_y1 = bubble.point.y + bubble.diameter;
            // Checks if player intersects with bubble. Return true for game_over.
            if b_x0 < p_x1 && b_x1 > p_x0 {
                if b_y1 > p_y0 {
                    return true;
                }
            }
        }
        false
    }

    pub fn start_next_level(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        if gamepad & wasm4::BUTTON_1 != 0 {
            self.setup_level();
        }
    }
    pub fn setup_level(&mut self) {
        self.player.reset();
        self.difficulty += 1;

        if self.difficulty == 1 {
            self.add_bubble(88, 90, 16, 1, true);
        }
        if self.difficulty == 2 {
            self.add_bubble(20, 10, 12, 2, true);
        }
        if self.difficulty == 3 {
            self.add_bubble(80, 0, 4, 2, false);
            self.add_bubble(136, 40 + 80, 8, 2, false);
        }
        if self.difficulty == 4 {
            self.add_bubble(80, 0, 4, 1, false);
            self.add_bubble(136, 40 + 80, 8, 1, true);
            self.add_bubble(40, 15, 8, 1, true);
            self.add_bubble(88, 90, 16, 1, false);
        }

        if self.difficulty == 5 {
            self.add_bubble(10, 20, 8, 1, true);
            self.add_bubble(10, 20, 8, 2, true);
            self.add_bubble(20, 0, 4, 2, false);
            self.add_bubble(90, 150, 4, 2, true);
        }

        if self.difficulty == 6 {
            self.add_bubble(40, 15, 8, 1, true);
            self.add_bubble(88, 90, 16, 1, true);
            self.add_bubble(90, 150, 4, 1, true);
            self.add_bubble(30, 35, 8, 1, false);
            self.add_bubble(40, 155, 8, 1, true);
            self.add_bubble(40, 15, 8, 1, true);
			self.add_bubble(10, 120, 8, 1, false);
			self.add_bubble(20, 100, 4, 1, false);
        }

        if self.difficulty == 1 {
		}
        // self.add_bubble(10, 20, 8, 2, true);
        // self.add_bubble(20, 0, 4, 2, false);
        // self.add_bubble(90, 150, 4, 2, true);

        // self.add_bubble(30, 35, 8, 1, false);
        // self.add_bubble(40, 15, 8, 1, true);
        // self.add_bubble(88, 90, 16, 1, false);
        // self.add_bubble(90, 150, 4, 2, true);

        // self.add_bubble(30, 35, 8, 1, false);
        // self.add_bubble(40, 15, 8, 1, true);
        // self.add_bubble(88, 90, 16, 1, false);
        // self.add_bubble(90, 150, 4, 2, true);

        // self.add_bubble(30, 35, 8, 1, false);
        // self.add_bubble(40, 15, 8, 1, true);
        // self.add_bubble(88, 90, 16, 1, false);
        // self.add_bubble(90, 150, 4, 2, true);
        self.game_state = GameState::Playing;
    }
}
