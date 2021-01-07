use crate::maze::*;
use rand::Rng;

pub fn generate(w: usize, h: usize) -> Maze {
	let mut rng = rand::thread_rng();
	let mut maze = Maze::new(w, h, true);

	for x in 0..w-1 {
		for y in 0..h-1 {
			let dir = if rng.gen() {Dir::Right} else {Dir::Down};
			maze[(dir, (x, y))] = false;
		}
	}

	maze
}
