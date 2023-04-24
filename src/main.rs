#![allow(non_upper_case_globals)]

use std::{process::exit, f64::consts::PI};

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
	current_direction: Direction,
	collided_squares: Vec<usize>
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
			current_direction: Direction::Right,
			collided_squares: Vec::new()
		};
	}

	fn draw(&self) {

		for i in 0..self.squares.len() {
			if self.collided_squares.contains(&i) {
				draw_rectangle(self.squares[i].x, self.squares[i].y, self.square_size, self.square_size, ORANGE);
			} else {
				draw_rectangle(self.squares[i].x, self.squares[i].y, self.square_size, self.square_size, self.squares[i].color);
			}
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

	fn snake_collision(&mut self) -> bool {
		for i in 0..self.squares.len() {
			for j in 0..self.squares.len() {
				if i != j {
					if self.squares[i].x == self.squares[j].x && self.squares[i].y == self.squares[j].y {
						self.collided_squares = Vec::from([i, j]);
						return true;
					}
				}
			}
		}
		return false;
	}

	fn wall_collision(&mut self) -> bool {
		for i in 0..self.squares.len() {
			if self.squares[i].x < 0.0 || self.squares[i].y < 0.0 || self.squares[i].y+self.square_size > screen_height() || self.squares[i].x+self.square_size > screen_width() {
				self.collided_squares = Vec::from([i]);
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

			let x = rand::gen_range(0, self.columns) * self.apple_size + self.start_x;
			let y = rand::gen_range(0, self.rows) * self.apple_size + self.start_y;

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

struct Button {
	x: i32,
	y: i32,
	text: String,
	text_size: u16,
	padding: i32,
	text_dimensions: TextDimensions,
	hover: bool
}

impl Button {
	fn new(x: i32, y: i32, label: String, text_size: u16, padding: i32) -> Self {
		return Self {
			x: x,
			y: y,
			text: label.clone(),
			text_size: text_size,
			padding: padding,
			text_dimensions: measure_text(&label, None, text_size, 1.0),
			hover: false
		};
	}

	fn draw(&self) {
		
		let text_size = self.text_dimensions;

		let color = match self.hover {
			false => WHITE,
			true => Color { r: 0.875, g: 0.875, b: 0.875, a: 1.0 }
		};

		draw_rectangle_lines(self.x as f32-self.padding as f32-text_size.width/2.0, self.y as f32 - text_size.height/2.0 as f32-self.padding as f32, text_size.width+self.padding as f32*2.0, text_size.height+self.padding as f32*2.0, 5.0, color);
		draw_text(&self.text, self.x as f32 - text_size.width/2.0, self.y as f32 + text_size.height/2.0, self.text_size as f32, color);

	}

	fn is_over(&self, x: i32, y: i32) -> bool {
		if x > self.x - self.padding - self.text_dimensions.width as i32/2 && x < self.x + self.padding + self.text_dimensions.width as i32/2 {
			if y > self.y - self.padding - self.text_dimensions.height as i32/2 && y < self.y + self.padding + self.text_dimensions.height as i32/2 {
				return true;
			}
		}
		return false;
	}

}

#[cfg(target_family = "wasm")]
fn on_web() -> bool {
	return true;
}

#[cfg(target_family = "windows")]
fn on_web() -> bool {
	return false;
}

fn ease(x: f64) -> f64 {
	return ((x * PI) / 2.0).cos();
}

fn calculate_speed(score: u64) -> f64 {
	let speed = 0.15 * ease(score as f64 / 100.0);
	if speed < 0.0 {
		return 0.0;
	}
	return speed;
}

#[macroquad::main(window_conf)]
async fn main() {

	if on_web() {
		info!("WASM detected.");
	}

	info!("Setting up.");

	// info!("Initializing direction queue.");
	let mut direction_queue: Vec<Direction> = Vec::new();

	// info!("Initializing snake vector.");
	let mut snake = Snake::new(square_size);

	// info!("Initializing apples vector.");

	let mut apples = Apples::new(square_size as i32);
	apples.random();
	apples.random();

	let mut game_over: bool = false;
	let mut in_game: bool = false;
	let mut score: u64 = 0;

	let mut start_button = Button::new(screen_width().round() as i32/2, screen_height().round() as i32/5 * 3, "Start".to_string(), 75, 10);
	let mut exit_button = Button::new(screen_width().round() as i32/2, screen_height().round() as i32 / 7 * 5, "Exit".to_string(), 75, 10);

	let menu_logo: Texture2D = Texture2D::from_file_with_format(include_bytes!(".\\menu.png"), Some(ImageFormat::Png));
	menu_logo.set_filter(FilterMode::Nearest);

	loop {
		clear_background(BLACK);

		println!("{}", ease(score as f64 / 100.0));

		if !game_over && in_game {
			if is_key_pressed(KeyCode::W) {
				direction_queue.push(Direction::Up);
			} else if is_key_pressed(KeyCode::D) {
				direction_queue.push(Direction::Right);
			} else if is_key_pressed(KeyCode::A) {
				direction_queue.push(Direction::Left);
			} else if is_key_pressed(KeyCode::S) {
				direction_queue.push(Direction::Down);
			}

			if get_time() - snake.last_move > calculate_speed(score) {
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
					score += 1;
					break;
				}
			}

			if snake.snake_collision() || snake.wall_collision() {
				game_over = true;
				score = 0;
			}

		} else if !in_game {

			if is_mouse_button_pressed(MouseButton::Left) {
				let pos = mouse_position();
				if start_button.is_over(pos.0 as i32, pos.1 as i32) {
					in_game = true;
				}
			} else {
				let pos = mouse_position();
				if start_button.is_over(pos.0 as i32, pos.1 as i32) {
					start_button.hover = true;
				} else {
					start_button.hover = false;
				}
			}

			if is_mouse_button_pressed(MouseButton::Left) {
				let pos = mouse_position();
				if exit_button.is_over(pos.0 as i32, pos.1 as i32) {
					info!("Requested exit.");
					exit(0);
				}
			} else {
				let pos = mouse_position();
				if exit_button.is_over(pos.0 as i32, pos.1 as i32) {
					exit_button.hover = true;
				} else {
					exit_button.hover = false;
				}
			}

			draw_texture_ex(menu_logo, screen_width()/2.0-320.0, screen_height()/3.0-180.0, WHITE, DrawTextureParams { dest_size: Some(vec2(640.0, 360.0)), ..Default::default() });

			start_button.draw();
			if !on_web() {
				exit_button.draw();
			}


		} else {
			if is_key_pressed(KeyCode::Space) {

				snake = Snake::new(square_size);

				apples = Apples::new(square_size as i32);
				apples.random();
				apples.random();

				game_over = false;
				in_game = false;
			}

			let text_size = measure_text("Oh no! Press space to continue", None, 32, 1.0);
			draw_text("Oh no! Press space to continue", screen_width()/2.0 - text_size.width/2.0, screen_height()/2.0 - text_size.height/2.0, 32.0, WHITE);

		}

		if in_game {
			snake.draw();
			apples.draw();

			let text_size = measure_text(format!("Score: {}", score).as_str(), None, 64, 1.0);
			draw_text(format!("Score: {}", score).as_str(), screen_width()/2.0 - text_size.width/2.0, text_size.height, 64.0, WHITE);

		}

		next_frame().await
	}
}


