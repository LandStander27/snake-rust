#![allow(non_upper_case_globals)]

use macroquad::prelude::*;

fn window_conf() -> Conf {
	Conf {
	    window_title: "Snake".to_owned(),
	    fullscreen: true,
	    ..Default::default()
	}
}

const square_size: f32 = 20.0;

#[derive(PartialEq)]
enum Direction {
	Up,
	Down,
	Left,
	Right
}

struct Square {
	x: f32,
	y: f32,
	color: Color
}

impl Square {
	fn new(x: f32, y: f32, color: Color) -> Self {
		return Self {
			x: x,
			y: y,
			color: color
		};
	}
}

struct Snake {
	squares: Vec<Square>,
	square_size: f32,
	last_move: f64,
	current_direction: Direction
}

impl Snake {
	fn new(size: f32) -> Self {

		let mut snake: Vec<Square> = Vec::new();
		snake.push(Square::new(screen_width()/2.0 - 8.0, screen_height()/2.0 - 8.0, GREEN));
		snake.push(Square::new(screen_width()/2.0 - 8.0 - size, screen_height()/2.0 - 8.0, GREEN));
		snake.push(Square::new(screen_width()/2.0 - 8.0 - size*2.0, screen_height()/2.0 - 8.0, GREEN));
		snake.reverse();

		return Self {
			squares: snake,
			square_size: size,
			last_move: get_time(),
			current_direction: Direction::Right
		};
	}

	fn draw(&self) {
		for i in &self.squares {
			draw_rectangle(i.x, i.y, self.square_size-1.0, self.square_size-1.0, i.color);
		}
	}

	fn move_snake(&mut self) {
		self.squares.remove(0);
		
		let mut offset: (f32, f32) = (0.0, 0.0);
		match self.current_direction {
			Direction::Up => offset.1 = -self.square_size,
			Direction::Down => offset.1 = self.square_size,
			Direction::Left => offset.0 = -self.square_size,
			Direction::Right => offset.0 = self.square_size
		}

		self.squares.push(Square::new(self.squares.last().unwrap().x + offset.0, self.squares.last().unwrap().y + offset.1, GREEN));
		self.last_move = get_time();

	}

}

#[macroquad::main(window_conf)]
async fn main() {

	println!("Setting up.");

	let mut direction_queue: Vec<Direction> = Vec::new();

	let mut snake = Snake::new(square_size);

	loop {
		clear_background(BLACK);

		if is_key_pressed(KeyCode::W) {
			direction_queue.push(Direction::Up);
		} else if is_key_pressed(KeyCode::D) {
			direction_queue.push(Direction::Right);
		} else if is_key_pressed(KeyCode::A) {
			direction_queue.push(Direction::Left);
		} else if is_key_pressed(KeyCode::S) {
			direction_queue.push(Direction::Down);
		}

		if get_time() - snake.last_move > 0.2 {
			if direction_queue.len() > 0 {
				let queue = direction_queue.remove(0);
				if match queue {
					Direction::Up => snake.current_direction != Direction::Down,
					Direction::Down => snake.current_direction != Direction::Up,
					Direction::Left => snake.current_direction != Direction::Right,
					Direction::Right => snake.current_direction != Direction::Left,
				} {
					snake.current_direction = queue;
				}
			}
			snake.move_snake();
		}

		snake.draw();

		next_frame().await
	}
}


