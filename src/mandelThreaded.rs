use std::sync::{Arc, Mutex};
use std::thread;

const MIN_X: f32 = -2.0;
const MAX_X: f32 = 0.5;
const MIN_Y: f32 = -1.25;
const MAX_Y: f32 = 1.25;
const WIDTH: i32 = 4000;
const HEIGHT: i32 = 4000;
const MAX_ITER: i32 = 255;

fn main() {
	let mut lookup: Vec<u8> = Vec::with_capacity((MAX_ITER + 1) as usize);
	build_table_colors(&mut lookup);
	let lookup = Arc::new(lookup);

	let length = (WIDTH * HEIGHT) as usize;
	let mut image_output: Vec<u8> = Vec::with_capacity(length);
	unsafe { image_output.set_len(length) };
	let image_output = Arc::new(Mutex::new(image_output));
	
	let counter = Arc::new(Mutex::new(0));

	let mut thread_pool = Vec::with_capacity(8);
	for _ in 0..8 {
		let counter = counter.clone();
		let lookup = lookup.clone();
		let image_output = image_output.clone();
		let t = thread::spawn(move || {
			let mut done = false;
			while !done {
				let local_counter = {
					let mut counter = counter.lock().unwrap();
					*counter += 1;
					*counter - 1
				};

				let x_pixel: i32 = local_counter % WIDTH;
				let y_pixel: i32 = local_counter / WIDTH;

				if y_pixel >= WIDTH {
					done = true;
				} else {
					let x0: f32 = pixels_to_complexes_x(x_pixel);
					let y0: f32 = pixels_to_complexes_y(y_pixel);

					let mut iterations: i32 = 0;

					let mut x: f32 = 0.; let mut y: f32 = 0.;
					let mut x_square: f32 = 0.; let mut y_square: f32 = 0.;

					while x_square + y_square <= 4. && iterations < MAX_ITER {
						y = 2.0 * x * y + y0;
						x = x_square - y_square + x0;

						x_square = x * x;
						y_square = y * y;

						iterations += 1;
					}
					{
						let mut image_output = image_output.lock().unwrap();
						image_output[local_counter as usize] = lookup[iterations as usize];
					}
				}
			}
		});
		thread_pool.push(t);
	}

	for i in thread_pool {
		i.join();
	}
}

fn pixels_to_complexes_x(x: i32) -> f32 {
	// Maps pixel 0 -> MIN_X; pixel WIDTH - 1 -> MAX_X
	(MAX_X - MIN_X) / (WIDTH - 1) as f32 * x as f32 + MIN_X
}

fn pixels_to_complexes_y(y: i32) -> f32 {
	// Maps pixel 0 -> MAX_Y; pixel HEIGHT - 1 -> MIN_Y
	(MIN_Y - MAX_Y) / (HEIGHT - 1) as f32 * y as f32 + MAX_Y
}

fn pick_color(iteration: i32) -> u8 {
	(255. / ((MAX_ITER - 2) * (iteration - 1)) as f32).round() as u8
}

fn build_table_colors(table: &mut Vec<u8>) {
	table.push(0);
	for i in 1..MAX_ITER {
		table.push(pick_color(i));
	}
	table.push(0);
}