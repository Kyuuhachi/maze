use crate::maze::*;
use rand::prelude::*;
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
	fn generate(&self, mut rng: &mut StdRng, size: Size) -> Maze {
		let mut maze = Maze::new(size, false);
		let mut poss = Vec::new();
		let mut uf: InPlaceUnificationTable<Key> = InPlaceUnificationTable::new();
		let keys = Array2::from_shape_fn(size, |pos| {
			poss.push((Dir::Down, pos));
			poss.push((Dir::Right, pos));
			uf.new_key(())
		});
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
