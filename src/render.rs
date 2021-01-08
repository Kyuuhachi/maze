use crate::maze::*;
use ndarray::Array2;

fn distance(
	maze: &Maze,
	centers: &Vec<Pos>,
) -> Array2<(usize, usize)> { // Which center (index in centers), and distance
    use std::collections::VecDeque;

	let mut distance = Array2::from_shape_simple_fn((maze.w(), maze.h()), || (0, 0));
	let mut seen = Array2::from_shape_simple_fn((maze.w(), maze.h()), || false);
	let mut queue = VecDeque::new();
	for (id, &pos) in centers.iter().enumerate() {
		queue.push_back((pos, id, 0));
	}

	while let Some((pos, id, dist)) = queue.pop_front() {
		if seen[pos] {
			continue
		} else {
			seen[pos] = true;
			distance[pos] = (id, dist);
		}

		for &dir in &Dir::ALL {
			if let Some(pos2) = maze.shift(dir, pos) {
				if maze[(dir, pos)] {
					queue.push_back((pos2, id, dist+1));
				}
			}
		}
	}

	distance
}

fn hsv2rgb(hue: f32, sat: f32, val: f32) -> image::Rgb<u8> {
	use std::f32::consts::TAU;
	let hav = |t: f32| (1. - f32::cos(t)) / 2.;

	let val = val*val.sqrt();
	let sat = sat.sqrt();
	let calc = |c: f32| (val * (1. - sat*hav((hue-c/3.) * TAU)) * 255.) as u8;
	return image::Rgb([calc(0.), calc(1.), calc(2.)]);
}

pub fn render(
	rng: &mut (impl rand::Rng + ?Sized),
	maze: &Maze,
	ncells: u32,
	color: (f32, f32),
) -> image::RgbImage {

	let centers = (0..ncells).map(|_| (
		rng.gen_range(0..maze.w()),
		rng.gen_range(0..maze.h()),
	)).collect();

	let dist = distance(&maze, &centers);

	let maxdist = dist.fold(0, |a, b| usize::max(a, b.1));

	let hues: Vec<f32> = (0..ncells).map(|_| rng.gen_range(color.0.min(color.1)..=color.0.max(color.1)) / 360.).collect();

	image::ImageBuffer::from_fn(dist.nrows() as u32,dist.ncols() as u32, |x, y| {
		let (which, d) = dist[(x as usize, y as usize)];
		let hue = hues[which];
		let fade = d as f32 / maxdist as f32;
		let sat = 0.2 + 0.8*fade;
		let val = 1.0 - 0.6*fade;
		hsv2rgb(hue, sat, val)
	})
}
