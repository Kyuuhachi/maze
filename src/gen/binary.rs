use crate::maze::*;
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
	Southeast,
	Southwest,
	Northwest,
	Northeast,
}

pub struct BinaryTree(pub Direction);
impl Generator for BinaryTree {
	fn generate(&self, rng: &mut StdRng, size: Size) -> Maze {
		let mut maze = Maze::new(size, true);

		let (vert, horz) = match self.0 {
			Direction::Southeast => (true,  true),
			Direction::Southwest => (true,  false),
			Direction::Northwest => (false, false),
			Direction::Northeast => (false, true),
		};

		let (hdir, hmin, hmax) =
			if horz {(Dir::Right, 0, maze.w()-1)}
			else    {(Dir::Left,  1, maze.w())};
		let (vdir, vmin, vmax) =
			if vert {(Dir::Down,  0, maze.h()-1)}
			else    {(Dir::Up,    1, maze.h())};

		for x in hmin..hmax {
			for y in vmin..vmax {
				let dir = if rng.gen() {vdir} else {hdir};
				maze[(dir, (x, y))] = false;
			}
		}

		maze
	}
}
