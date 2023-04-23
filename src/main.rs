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

		let columns = (screen_width() / size as f32) as i32;
		let start_x: i32 = (screen_width() % size as f32) as i32 / 2;

		let rows = (screen_height() / size as f32) as i32;
		let start_y: i32 = (screen_height() % size as f32) as i32 / 2;

		snake.push(Square::new((columns/2) as f32*size + start_x as f32, (rows/2) as f32*size + start_y as f32, GREEN));
		snake.push(Square::new((columns/2) as f32*size + start_x as f32 - size, (rows/2) as f32*size + start_y as f32, GREEN));
		snake.push(Square::new((columns/2) as f32*size + start_x as f32 - size*2.0, (rows/2) as f32*size + start_y as f32, GREEN));

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
			draw_rectangle(i.x, i.y, self.square_size, self.square_size, i.color);
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

	fn collision(&self, col: &Square) -> bool {
		for i in &self.squares {
			if col.x == i.x && col.y == i.y {
				return true;
			}
		}
		return false;
	}

	fn grow(&mut self) {
		let last_squares: (&Square, &Square) = (&self.squares[0], &self.squares[1]);
		let mut offset: (i32, i32) = (1, 1);

		if last_squares.0.x < last_squares.1.x {
			offset.0 = -1;
		}
		if last_squares.0.y < last_squares.1.y {
			offset.1 = -1;
		}
		
		if last_squares.0.x == last_squares.1.x {
			offset.0 = 0;
		}
		if last_squares.0.y == last_squares.1.y {
			offset.1 = 0;
		}

		let new: Square = Square::new(last_squares.0.x + offset.0 as f32*self.square_size, last_squares.0.y + offset.1 as f32*self.square_size, GREEN);
		self.squares.insert(0, new);

	}

}

struct Apples {
	apples: Vec<Square>,
	apple_size: i32,
	columns: i32,
	rows: i32,
	start_x: i32,
	start_y: i32,
}

impl Apples {
	fn new(size: i32) -> Self {

		let columns = (screen_width() / size as f32) as i32;
		let start_x: i32 = (screen_width() % size as f32) as i32 / 2;

		let rows = (screen_height() / size as f32) as i32;
		let start_y: i32 = (screen_height() % size as f32) as i32 / 2;

		return Self {
			apples: Vec::new(),
			apple_size: size,
			columns: columns,
			rows: rows,
			start_x: start_x,
			start_y: start_y
		};
	}

	fn random(&mut self) {

		loop {

			let x = rand::gen_range(0, self.columns+1) * self.apple_size + self.start_x;
			let y = rand::gen_range(0, self.rows+1) * self.apple_size + self.start_y;

			for i in &self.apples {
				if i.x == x as f32 && i.y == y as f32 {
					continue;
				}
			}

			self.apples.push(Square::new(x as f32, y as f32, RED));
			break;

		}

	}

	fn draw(&self) {
		for i in &self.apples {
			draw_rectangle(i.x, i.y, self.apple_size as f32, self.apple_size as f32, i.color);
		}
	}

}

#[macroquad::main(window_conf)]
async fn main() {

	println!("Setting up.");

	let mut direction_queue: Vec<Direction> = Vec::new();

	let mut snake = Snake::new(square_size);

	let mut apples = Apples::new(square_size as i32);
	apples.random();
	apples.random();

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

		if get_time() - snake.last_move > 0.15 {
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

		for i in 0..apples.apples.len() {
			if snake.collision(&apples.apples[i]) {
				apples.apples.remove(i);
				apples.random();
				snake.grow();
				break;
			}
		}

		snake.draw();
		apples.draw();

		next_frame().await
	}
}


