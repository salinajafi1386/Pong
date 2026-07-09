use macroquad::prelude::*;

const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const PADDLE_W: f32 = 12.0;
const PADDLE_H: f32 = 80.0;
const BALL_SIZE: f32 = 12.0;
const PADDLE_OFFSET: f32 = 20.0;
const PADDLE_SPEED: f32 = 400.0; // pixels per second
const WIN_SCORE: u32 = 5;

struct Paddle<'a> {
    rect: Rect,
    texture: &'a Texture2D,
}

impl<'a> Paddle<'a> {
    fn new(x: f32, texture: &'a Texture2D) -> Self {
        Self {
            rect: Rect::new(x, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H),
            texture,
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.rect.w, self.rect.h)),
                ..Default::default()
            },
        );
    }

    fn update(&mut self, dt: f32, going_up_key: KeyCode, going_down_key: KeyCode) {
        if is_key_down(going_down_key) {
            self.rect.y += PADDLE_SPEED * dt;
        }

        if is_key_down(going_up_key) {
            self.rect.y -= PADDLE_SPEED * dt;
        }

        self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - PADDLE_H);
    }
}

struct Ball<'a> {
    rect: Rect,
    vel: Vec2,
    texture: &'a Texture2D,
}

impl<'a> Ball<'a> {
    fn new(texture: &'a Texture2D) -> Self {
        Self {
            rect: Rect::new(
                WINDOW_W / 2.0 - BALL_SIZE / 2.0,
                WINDOW_H / 2.0 - BALL_SIZE / 2.0,
                BALL_SIZE,
                BALL_SIZE,
            ),
            vel: Vec2::new(300.0, 220.0),
            texture,
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.rect.w, self.rect.h)),
                ..Default::default()
            },
        );
    }

    fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt;
        self.rect.y += self.vel.y * dt;

        // bounce off top wall
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
            self.vel.y = self.vel.y.abs();
        }
        // bounce off bottom wall
        if self.rect.y + self.rect.h > WINDOW_H {
            self.rect.y = WINDOW_H - self.rect.h;
            self.vel.y = -self.vel.y.abs();
        }
    }

    fn check_paddles(&mut self, left: &Paddle, right: &Paddle) {
        if self.rect.overlaps(&left.rect) {
            self.rect.x = left.rect.x + left.rect.w; // push ball out
            self.vel.x = self.vel.x.abs();
        }

        if self.rect.overlaps(&right.rect) {
            self.vel.x = -self.vel.x.abs();
            self.rect.x = right.rect.x - self.rect.w; // push ball out
        }
    }

    fn reset(&mut self) {
        self.rect.x = WINDOW_W / 2.0 - BALL_SIZE / 2.0;
        self.rect.y = WINDOW_H / 2.0 - BALL_SIZE / 2.0;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        ..Conf::default()
    }
}

fn draw_centre_line() {
    let mut y = 10.0;
    while y < WINDOW_H {
        draw_line(WINDOW_W / 2.0, y, WINDOW_W / 2.0, y + 15.0, 2.0, DARKGRAY);
        y += 25.0;
    }
}

struct Score {
    left: u32,
    right: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { left: 0, right: 0 }
    }
}

enum GameState {
    Playing,
    GameOver,
}

impl Score {
    fn draw(&self) {
        let text = format!("{}   {}", self.left, self.right);
        let dims = measure_text(&text, None, 48, 1.0);
        draw_text(&text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);
    }

    fn update(&mut self, ball: &Ball) -> bool {
        let left_exit = ball.rect.x + ball.rect.w < 0.0;
        let right_exit = ball.rect.x > WINDOW_W;

        if left_exit {
            self.right += 1;
        }

        if right_exit {
            self.left += 1;
        }

        left_exit || right_exit
    }
}

struct Game<'a> {
    ball: Ball<'a>,
    left: Paddle<'a>,
    right: Paddle<'a>,
    score: Score,
    game_state: GameState,
    winner: String,
}

impl<'a> Game<'a> {
    fn new(paddle_texture: &'a Texture2D, ball_texture: &'a Texture2D) -> Self {
        let score = Score::default();
        let game_state = GameState::Playing;
        let winner = "".to_string();
        let ball = Ball::new(ball_texture);
        let left = Paddle::new(PADDLE_OFFSET, &paddle_texture);
        let right = Paddle::new(WINDOW_W - PADDLE_W - PADDLE_OFFSET, &paddle_texture);

        Self {
            ball: ball,
            left: left,
            right: right,
            score: score,
            game_state: game_state,
            winner: winner,
        }
    }

    fn update(&mut self, dt: f32, paddle_texture: &'a Texture2D) {
        match self.game_state {
            GameState::Playing => {
                self.left.update(dt, KeyCode::W, KeyCode::S);
                self.right.update(dt, KeyCode::Up, KeyCode::Down);
                self.ball.update(dt);
                self.ball.check_paddles(&self.left, &self.right);
                if self.score.update(&self.ball) {
                    self.ball.reset();
                    if self.score.left >= WIN_SCORE {
                        self.winner = "Left player wins!".to_string();
                        self.game_state = GameState::GameOver;
                    } else if self.score.right >= WIN_SCORE {
                        self.winner = "Right player wins!".to_string();
                        self.game_state = GameState::GameOver;
                    }
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::R) {
                    self.score = Score::default();
                    self.ball.reset();
                    self.left = Paddle::new(PADDLE_OFFSET, &paddle_texture);
                    self.right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W, &paddle_texture);
                    self.game_state = GameState::Playing;
                }
            }
        }
    }

    fn draw(&self) {
        match self.game_state {
            GameState::Playing => {
                clear_background(BLACK);
                draw_centre_line();

                self.left.draw();
                self.right.draw();
                self.ball.draw();
                self.score.draw();
            }
            GameState::GameOver => {
                let dims = measure_text(&self.winner, None, 48, 1.0);
                draw_text(
                    &self.winner,
                    WINDOW_W / 2.0 - dims.width / 2.0,
                    WINDOW_H / 2.0,
                    48.0,
                    WHITE,
                );

                let hint = "Press R to restart";
                let hdims = measure_text(hint, None, 24, 1.0);
                draw_text(
                    hint,
                    WINDOW_W / 2.0 - hdims.width / 2.0,
                    WINDOW_H / 2.0 + 40.0,
                    24.0,
                    GRAY,
                );
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let ball_texture = load_texture("assets/ball.png").await.unwrap();
    let paddle_texture = load_texture("assets/paddle.png").await.unwrap();

    let mut game = Game::new(&paddle_texture, &ball_texture);

    loop {
        let dt = get_frame_time();

        game.update(dt, &paddle_texture);

        game.draw();

        next_frame().await;
    }
}
