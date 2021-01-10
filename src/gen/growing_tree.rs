use crate::maze::*;
use ndarray::Array2;
use rand::prelude::*;

pub struct Backtrack;
impl Generator for Backtrack {
	fn generate(&self, rng: &mut StdRng, size: Size) -> Maze {
		growing_tree(rng, size, &mut Vec::new(), &mut Vec::push, &mut Vec::pop)
	}
}

pub struct PrimTrue;
impl Generator for PrimTrue {
	fn generate(&self, rng: &mut StdRng, size: Size) -> Maze {
		use std::collections::BinaryHeap;
		let weight: Array2<u32> = Array2::from_shape_simple_fn(size, ||rng.gen());
		growing_tree(rng, size,
			&mut BinaryHeap::new(),
			&mut |heap, pos| heap.push((weight[pos], pos)),
			&mut |heap| heap.pop().map(|a|a.1),
		)
	}
}

pub struct PrimSimplified;
impl Generator for PrimSimplified {
	fn generate(&self, rng: &mut StdRng, size: Size) -> Maze {
		let mut rng2 = StdRng::from_seed({
			let mut seed = Default::default();
			rng.fill(&mut seed);
			seed
		});

		growing_tree(rng, size, &mut Vec::new(), &mut |vec, v| {
			vec.push(v);
			let l = vec.len();
			vec.swap(rng2.gen_range(0..l), l - 1);
		}, &mut Vec::pop)
	}
}

fn growing_tree<T>(
	mut rng: &mut StdRng,
	size: Size,
	state: &mut T,
	push: &mut impl FnMut(&mut T, Pos),
	pop: &mut impl FnMut(&mut T) -> Option<Pos>,
) -> Maze {
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
					// Pushing pos before dest would be closer to real DFS behavior, but the
					// images created this way look better.
					push(state, dest);
					push(state, pos);
					break;
				}
			}
		}
	}
	maze
}
