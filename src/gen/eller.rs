use crate::maze::*;
use super::kruskal::Key;
use rand::prelude::*;
use ena::unify::InPlaceUnificationTable;
use ndarray::Array2;
use std::collections::HashMap;

pub struct Eller;
impl Generator for Eller {
	fn generate(&self, rng: &mut StdRng, size: Size) -> Maze {
		let mut maze = Maze::new(size, false);

		let mut uf: InPlaceUnificationTable<Key> = InPlaceUnificationTable::new();
		let keys = Array2::from_shape_fn(size, |_| uf.new_key(()));

		for y in 0..maze.h() {
			let mut xs: Vec<_> = (0..maze.w()).collect();
			xs.shuffle(rng);
			for x in xs {
				let pos = (x, y);
				if let Some(pos2) = maze.shift(Dir::Right, pos) {
					if !uf.unioned(keys[pos], keys[pos2]) && (rng.gen() || y == maze.h()-1) {
						uf.union(keys[pos], keys[pos2]);
						maze[(Dir::Right, pos)] = true;
					}
				}
			}

			let mut sets = HashMap::new();
			for pos in (0..maze.w()).map(|x| (x, y)) {
				let Key(key) = uf.find(keys[pos]);
				sets.entry(key).or_insert_with(Vec::new).push(pos);
			}
			for val in sets.values() {
				let &n = val.choose(rng).unwrap();
				for &pos in val {
					if let Some(pos2) = maze.shift(Dir::Down, pos) {
						if pos == n || rng.gen() {
							uf.union(keys[pos], keys[pos2]);
							maze[(Dir::Down, pos)] = true;
						}
					}
				}
			}
		}

		maze
	}
}

