use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_coords: (i32, i32),
    dimensions: (i32, i32),
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(dimensions: (i32, i32)) -> Self {
        Game {
            snake: Snake::new(2, 2),
            food_coords: (6, 4),
            dimensions: dimensions,
            food_exists: true,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        if let Some(direction) = direction {
            if direction == self.snake.head_direction().opposite() {
                return;
            }
        }

        self.update_snake(direction);
    }

    pub fn draw(&self, context: &Context, g_buf: &mut G2d) {
        self.snake.draw(context, g_buf);

        if self.food_exists {
            let (x, y) = self.food_coords;
            draw_block(FOOD_COLOR, (x, y), context, g_buf);
        }

        let (width, height) = self.dimensions;
        draw_rectangle(BORDER_COLOR, (0, 0), (width, 1), context, g_buf);
        draw_rectangle(BORDER_COLOR, (0, height - 1), (width, 1), context, g_buf);
        draw_rectangle(BORDER_COLOR, (0, 0), (1, height), context, g_buf);
        draw_rectangle(BORDER_COLOR, (width - 1, 0), (1, height), context, g_buf);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, (0, 0), (width, height), context, g_buf);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        if self.food_exists && self.food_coords == self.snake.head_position() {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_snake_alive(&self, direction: Option<Direction>) -> bool {
        let next_head = self.snake.next_head(direction);
        let (next_x, next_y) = next_head;
        let (width, height) = self.dimensions;

        if self.snake.overlap_tail(next_head) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < width - 1 && next_y < height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let (width, height) = self.dimensions;
        let mut new_x = rng.gen_range(1..width - 1);
        let mut new_y = rng.gen_range(1..height - 1);
        while self.snake.overlap_tail((new_x, new_y)) {
            new_x = rng.gen_range(1..width - 1);
            new_y = rng.gen_range(1..height - 1);
        }

        self.food_coords = (new_x, new_y);
        self.food_exists = true;
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        match self.check_snake_alive(direction) {
            true => {
                self.snake.move_forward(direction);
                self.check_eating();
            }
            false => {
                self.game_over = true;
            }
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_coords = (6, 4);
        self.game_over = false;
    }
}
