use std::fmt;
use ndarray::*;

#[derive(Clone, Copy, Debug)]
pub enum Dir { Right, Down, Left, Up }

impl Dir {
	pub const ALL: [Dir; 4] = [Dir::Right, Dir::Down, Dir::Left, Dir::Up];
}

#[derive(Clone, Copy, Debug)]
pub enum Dir2 { Right, Down }

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
			Dir::Left  => if x == 0 {None}  else {Some((x-1,y))},
			Dir::Up    => if y == 0 {None} else {Some((x,y-1))},
		}
	}

	fn resolve(&self, dir: Dir, pos: Pos) -> (Dir2, Pos) {
		match dir {
			Dir::Right => (Dir2::Right, pos),
			Dir::Down  => (Dir2::Down,  pos),
			Dir::Left  => (Dir2::Right, self.shift(dir, pos).unwrap()),
			Dir::Up    => (Dir2::Down,  self.shift(dir, pos).unwrap()),
		}
	}

	pub fn width(&self) -> usize { self.data.nrows() }
	pub fn height(&self) -> usize { self.data.ncols() }
}

impl std::ops::Index<(Dir, Pos)> for Maze {
	type Output = bool;

	fn index(&self, (dir, (x, y)): (Dir, Pos)) -> &bool {
		if self.shift(dir, (x, y)).is_none() { return &false; }
		let (dir, (x, y)) = self.resolve(dir, (x, y));

		match dir {
			Dir2::Right => &self.data[[x,y]].right,
			Dir2::Down  => &self.data[[x,y]].down,
		}
	}
}

impl std::ops::IndexMut<(Dir, Pos)> for Maze {
	fn index_mut(&mut self, (dir, (x, y)): (Dir, Pos)) -> &mut bool {
		std::assert!(self.shift(dir, (x, y)).is_some());
		let (dir, (x, y)) = self.resolve(dir, (x, y));

		match dir {
			Dir2::Right => &mut self.data[[x,y]].right,
			Dir2::Down  => &mut self.data[[x,y]].down,
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
