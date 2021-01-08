use crate::maze::*;
use rand::Rng;

// Intentionally leaving this one to one orientation only
pub fn generate(size: Size) -> Maze {
	let mut rng = rand::thread_rng();
	let mut maze = Maze::new(size, true);

	let diffusion = if rng.gen() {4} else {1};

	for y in 1..maze.h() {
		let mut i = 0;
		for x in 0..maze.w()-1 {
			maze[(Dir::Up, (x, y))] = false;
			if x == maze.w()-1 {
				maze[(Dir::Up, (rng.gen_range(i..=x), y))] = true;
			} else if rng.gen_range(0..=diffusion) == 0 {
				maze[(Dir::Right, (x, y))] = false;
				maze[(Dir::Up, (rng.gen_range(i..=x), y))] = true;
				i = x+1;
			}
		}
	}

	maze
}
