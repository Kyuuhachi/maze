use crate::maze::*;
use rand::Rng;

pub fn generate(w: usize, h: usize) -> Maze {
	let mut rng = rand::thread_rng();
	let mut maze = Maze::new(w, h, true);

	let (hdir, hmin, hmax) = if rng.gen() {(Dir::Right, 0, w-1)} else {(Dir::Left, 1, w)};
	let (vdir, vmin, vmax) = if rng.gen() {(Dir::Down,  0, h-1)} else {(Dir::Up,   1, h)};
	for x in hmin..hmax {
		for y in vmin..vmax {
			let dir = if rng.gen() {vdir} else {hdir};
			maze[(dir, (x, y))] = false;
		}
	}

	maze
}
