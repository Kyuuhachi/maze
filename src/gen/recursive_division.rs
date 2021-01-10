use crate::maze::*;
use std::cmp::Ordering;
use rand::prelude::*;

struct Rect(usize, usize, usize, usize);

pub struct RecursiveDivision;
impl Generator for RecursiveDivision {
	fn generate(&self, rng: &mut StdRng, size: Size) -> Maze {
		let mut maze = Maze::new(size, true);
		let mut rects = Vec::new();
		rects.push(Rect(0, 0, maze.w(), maze.h()));
		while let Some(Rect(x1, y1, x2, y2)) = rects.pop() {
			if x2-x1 < 2 || y2-y1 < 2 { continue }

			let horizontal = match usize::cmp(&(x2-x1), &(y2-y1)) {
				Ordering::Less => false,
				Ordering::Equal => rng.gen(),
				Ordering::Greater => true,
			};

			if horizontal {
				let x = rng.gen_range(x1+1..x2);
				let hole = rng.gen_range(y1..y2);
				for y in y1..y2 {
					maze[(Dir::Left, (x, y))] = y == hole;
				}
				rects.push(Rect(x1, y1, x, y2));
				rects.push(Rect(x, y1, x2, y2));
			} else {
				let y = rng.gen_range(y1+1..y2);
				let hole = rng.gen_range(x1..x2);
				for x in x1..x2 {
					maze[(Dir::Up, (x, y))] = x == hole;
				}
				rects.push(Rect(x1, y, x2, y2));
				rects.push(Rect(x1, y1, x2, y));
			}
		}
		maze
	}
}
