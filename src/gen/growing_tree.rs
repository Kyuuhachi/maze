use crate::maze::*;
use ndarray::Array2;
use rand::{Rng, seq::SliceRandom};

pub struct BackTrack;
impl Generator for BackTrack {
	fn generate(&self, size: Size) -> Maze {
		growing_tree(size, &mut Vec::new(), Vec::push, Vec::pop)
	}
}

pub struct PrimTrue;
impl Generator for PrimTrue {
	fn generate(&self, size: Size) -> Maze {
		use std::collections::BinaryHeap;
		let mut rng = rand::thread_rng();
		let weight: Array2<u32> = Array2::from_shape_simple_fn(size, ||rng.gen());
		growing_tree(size,
			&mut BinaryHeap::new(),
			|heap, pos| heap.push((weight[pos], pos)),
			|heap| heap.pop().map(|a|a.1),
		)
	}
}

pub struct PrimSimplified;
impl Generator for PrimSimplified {
	fn generate(&self, size: Size) -> Maze {
		growing_tree(size, &mut Vec::new(), |vec, v| {
			vec.push(v);
			let l = vec.len();
			vec.swap(rand::thread_rng().gen_range(0..l), l - 1);
		}, Vec::pop)
	}
}

// This function is technically broken: it opens its neighbors before they are actually visited,
// which makes the mazes "fuzzier". With my rendering method that makes it look better though, so
// I'm keeping it. A corrected version sits unused below.
fn growing_tree<T>(
	size: Size,
	state: &mut T,
	push: impl Fn(&mut T, Pos),
	pop: impl Fn(&mut T) -> Option<Pos>,
) -> Maze {
	let mut rng = rand::thread_rng();
	let mut maze = Maze::new(size, false);
	let mut seen = Array2::from_shape_simple_fn(size, || false);

	let start = (rng.gen_range(0..maze.w()), rng.gen_range(0..maze.h()));
	seen[start] = true;
	push(state, start);

	while let Some(pos) = pop(state) {
		let mut dirs = Dir::ALL;
		dirs.shuffle(&mut rng);
		for &dir in &dirs {
			if let Some(dest) = maze.shift(dir, pos) {
				if !seen[dest] {
					maze[(dir, pos)] = true;
					seen[dest] = true;
					push(state, dest);
				}
			}
		}
	}
	maze
}

fn _growing_tree2<T>(
	size: Size,
	state: &mut T,
	push: impl Fn(&mut T, (Dir, Pos)),
	pop: impl Fn(&mut T) -> Option<(Dir, Pos)>,
) -> Maze {
	let mut rng = rand::thread_rng();
	let mut maze = Maze::new(size, false);
	let mut seen = Array2::from_shape_simple_fn(size, || false);

	let start = (rng.gen_range(0..maze.w()), rng.gen_range(0..maze.h()));
	let mut dirs = Dir::ALL;
	dirs.shuffle(&mut rng);
	for &dir in &dirs {
		push(state, (dir, start));
	}

	while let Some((dir1, pos)) = pop(state) {
		if seen[pos] { continue; }
		seen[pos] = true;
		maze[(dir1, pos)] = true;

		let mut dirs = Dir::ALL;
		dirs.shuffle(&mut rng);
		for &dir in &dirs {
			if let Some(dest) = maze.shift(dir, pos) {
				push(state, (match dir {
					Dir::Right => Dir::Left,
					Dir::Down => Dir::Up,
					Dir::Left => Dir::Right,
					Dir::Up => Dir::Down,
				}, dest));
			}
		}
	}

	maze
}
