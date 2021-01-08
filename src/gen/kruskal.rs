use crate::maze::*;
use rand::seq::SliceRandom;
use ena::unify::InPlaceUnificationTable;
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Key(u32);
impl ena::unify::UnifyKey for Key {
	type Value = ();
	fn index(&self) -> u32 { self.0 }
	fn from_index(u: u32) -> Self { Key(u) }
	fn tag() -> &'static str { "Key" }
}

pub struct Kruskal;
impl Generator for Kruskal {
	fn generate(&self, mut rng: &mut (impl rand::Rng + ?Sized), size: Size) -> Maze {
		let mut maze = Maze::new(size, false);
		let mut poss = Vec::new();
		let mut uf: InPlaceUnificationTable<Key> = InPlaceUnificationTable::new();
		let mut keys = Array2::from_shape_simple_fn(size, || Key(0));
		for x in 0..maze.w() {
			for y in 0..maze.h() {
				keys[(x, y)] = uf.new_key(());
				for &dir in &Dir::ALL {
					poss.push((dir, (x, y)));
				}
			}
		}
		poss.shuffle(&mut rng);
		for &(dir, pos) in &poss {
			if let Some(pos2) = maze.shift(dir, pos) {
				if !uf.unioned(keys[pos], keys[pos2]) {
					uf.union(keys[pos], keys[pos2]);
					maze[(dir, pos)] = true;
				}
			}
		}
		maze
	}
}
