use std::fmt;
use ndarray::*;

#[derive(Clone, Copy, Debug)]
pub enum Dir { Right, Down, Left, Up }

impl Dir {
	pub const ALL: [Dir; 4] = [Dir::Right, Dir::Down, Dir::Left, Dir::Up];
}

pub type Pos = (usize, usize);

struct Cell { right: bool, down: bool }

pub struct Maze {
	data: Array2<Cell>,
}

impl Maze {
	pub fn new(w: usize, h: usize, open: bool) -> Self {
		Maze {
			data: Array2::from_shape_simple_fn((w, h), || Cell {right: open, down: open})
		}
	}

	pub fn shift(&self, dir: Dir, (x, y): Pos) -> Option<Pos> {
		match dir {
			Dir::Right => if x == self.width()-1  {None} else {Some((x+1,y))},
			Dir::Down  => if y == self.height()-1 {None} else {Some((x,y+1))},
			Dir::Left  => if x == 0 {None} else {Some((x-1,y))},
			Dir::Up    => if y == 0 {None} else {Some((x,y-1))},
		}
	}

	pub fn width(&self) -> usize { self.data.nrows() }
	pub fn height(&self) -> usize { self.data.ncols() }
}

impl std::ops::Index<(Dir, Pos)> for Maze {
	type Output = bool;

	fn index(&self, (dir, pos): (Dir, Pos)) -> &bool {
		match self.shift(dir, pos) {
			None => &false,
			Some(pos2) => match dir {
				Dir::Right => &self.data[pos].right,
				Dir::Down  => &self.data[pos].down,
				Dir::Left  => &self.data[pos2].right,
				Dir::Up    => &self.data[pos2].down,
			}
		}
	}
}

impl std::ops::IndexMut<(Dir, Pos)> for Maze {
	fn index_mut(&mut self, (dir, pos): (Dir, Pos)) -> &mut bool {
		match self.shift(dir, pos) {
			None => panic!("Cannot open edges"),
			Some(pos2) => match dir {
				Dir::Right => &mut self.data[pos].right,
				Dir::Down  => &mut self.data[pos].down,
				Dir::Left  => &mut self.data[pos2].right,
				Dir::Up    => &mut self.data[pos2].down,
			}
		}
	}
}

impl std::fmt::Display for Maze {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "▄")?;
		for x in 0..self.width() {
			write!(f, "{}", if self[(Dir::Up, (x, 0))] {" "} else {"▄"})?;
			write!(f, "▄")?;
		}

		for y in 0..self.height() {
			write!(f, "\n")?;
			write!(f, "{}", if self[(Dir::Left, (0, y))] {"▄"} else {"█"})?;
			for x in 0..self.width() {
				write!(f, "{}", if self[(Dir::Down, (x, y))] {" "} else {"▄"})?;
				write!(f, "{}", if self[(Dir::Right, (x, y))] {"▄"} else {"█"})?;
			}
		}

		Ok(())
	}
}
