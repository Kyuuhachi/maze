use std::fmt;
use ndarray::Array2;

#[derive(Clone, Copy, Debug)]
pub enum Dir { Right, Down, Left, Up }

impl Dir {
	pub const ALL: [Dir; 4] = [Dir::Right, Dir::Down, Dir::Left, Dir::Up];
}

pub type Pos = (usize, usize);

pub type Size = (usize, usize);

pub trait Generator {
	fn generate(&self, rng: &mut (impl rand::Rng + ?Sized), size: Size) -> Maze;
}

pub struct Maze {
	// (right, down)
	data: Array2<(bool, bool)>,
}

impl Maze {
	pub fn new(size: Size, open: bool) -> Self {
		Maze {
			data: Array2::from_shape_simple_fn(size, || (open, open))
		}
	}

	pub fn shift(&self, dir: Dir, (x, y): Pos) -> Option<Pos> {
		match dir {
			Dir::Right => if x == self.w()-1 {None} else {Some((x+1,y))},
			Dir::Down  => if y == self.h()-1 {None} else {Some((x,y+1))},
			Dir::Left  => if x == 0 {None} else {Some((x-1,y))},
			Dir::Up    => if y == 0 {None} else {Some((x,y-1))},
		}
	}

	pub fn w(&self) -> usize { self.data.nrows() }
	pub fn h(&self) -> usize { self.data.ncols() }
}

impl std::ops::Index<(Dir, Pos)> for Maze {
	type Output = bool;

	fn index(&self, (dir, pos): (Dir, Pos)) -> &bool {
		match self.shift(dir, pos) {
			None => &false,
			Some(pos2) => match dir {
				Dir::Right => &self.data[pos].0,
				Dir::Down  => &self.data[pos].1,
				Dir::Left  => &self.data[pos2].0,
				Dir::Up    => &self.data[pos2].1,
			}
		}
	}
}

impl std::ops::IndexMut<(Dir, Pos)> for Maze {
	fn index_mut(&mut self, (dir, pos): (Dir, Pos)) -> &mut bool {
		match self.shift(dir, pos) {
			None => panic!("Cannot open edges"),
			Some(pos2) => match dir {
				Dir::Right => &mut self.data[pos].0,
				Dir::Down  => &mut self.data[pos].1,
				Dir::Left  => &mut self.data[pos2].0,
				Dir::Up    => &mut self.data[pos2].1,
			}
		}
	}
}

impl std::fmt::Display for Maze {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "▄")?;
		for x in 0..self.w() {
			write!(f, "{}", if self[(Dir::Up, (x, 0))] {" "} else {"▄"})?;
			write!(f, "▄")?;
		}

		for y in 0..self.h() {
			write!(f, "\n")?;
			write!(f, "{}", if self[(Dir::Left, (0, y))] {"▄"} else {"█"})?;
			for x in 0..self.w() {
				write!(f, "{}", if self[(Dir::Down, (x, y))] {" "} else {"▄"})?;
				write!(f, "{}", if self[(Dir::Right, (x, y))] {"▄"} else {"█"})?;
			}
		}

		Ok(())
	}
}
