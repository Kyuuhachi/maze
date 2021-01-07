use crate::maze::*;
use ndarray::Array2;
use rand::{Rng, seq::SliceRandom};

pub fn backtrack(w: usize, h: usize) -> Maze {
	growing_tree(w, h, &mut Vec::new(), Vec::push, Vec::pop)
}

pub fn prim_simplified(w: usize, h: usize) -> Maze {
	growing_tree(w, h, &mut Vec::new(), |vec, v| {
		vec.push(v);
		let l = vec.len();
		vec.swap(rand::thread_rng().gen_range(0..l), l - 1);
	}, Vec::pop)
}

pub fn prim_true(w: usize, h: usize) -> Maze {
	use std::collections::BinaryHeap;
	use ndarray_rand::RandomExt;
	use ndarray_rand::rand_distr::Uniform;
	let weight = Array2::random((w, h), Uniform::new_inclusive(u32::MIN, u32::MAX));
	growing_tree(w, h,
		&mut BinaryHeap::new(),
		|heap, pos| heap.push((weight[pos], pos)),
		|heap| heap.pop().map(|a|a.1),
	)
}

pub fn growing_tree2<T, Push, Pop>(
	w: usize,
	h: usize,
	state: &mut T,
	push: impl Fn(&mut T, (Dir, Pos)),
	pop: impl Fn(&mut T) -> Option<(Dir, Pos)>,
) -> Maze {
	let mut rng = rand::thread_rng();
	let mut maze = Maze::new(w, h, false);
	let mut seen: Array2<bool> = Array2::from_shape_simple_fn((w, h), || false);

	let start = (rng.gen_range(0..w), rng.gen_range(0..h));
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

pub fn growing_tree<T>(
	w: usize,
	h: usize,
	state: &mut T,
	push: impl Fn(&mut T, Pos),
	pop: impl Fn(&mut T) -> Option<Pos>,
) -> Maze {
	let mut rng = rand::thread_rng();
	let mut maze = Maze::new(w, h, false);
	let mut seen: Array2<bool> = Array2::from_shape_simple_fn((w, h), || false);

	let start = (rng.gen_range(0..w), rng.gen_range(0..h));
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
