mod maze;
mod render;
mod gen {
	pub mod growing_tree;
	pub mod recursive_division;
	pub mod kruskal;
	pub mod binary;
	pub mod sidewinder;
}

use rand::prelude::*;
use maze::Generator;

macro_rules! time {
	($x:literal, $y:expr) => { {
		use std::time::Instant;
		let time1 = Instant::now();
		let v = $y;
		let time2 = Instant::now();
		println!("{:10} {:?}", $x, time2 - time1);
		v
	} };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// let gen = gen::growing_tree::BackTrack;
	// let gen = gen::growing_tree::PrimSimplified;
	let gen = gen::growing_tree::PrimTrue;
	// let gen = gen::recursive_division::RecursiveDivision;
	// let gen = gen::kruskal::Kruskal;
	// let gen = gen::binary::BinaryTree;
	// let gen = gen::sidewinder::SideWinder;
	let mut rng = rand::rngs::StdRng::from_entropy();

	time!("Total", {
		let maze = time!("Generation", gen.generate(&mut rng, (1920, 1080)));
		let img = time!("Rendering", render::render(&mut rng, &maze, 64, 0., 1.0));
		time!("Saving", img.save("test.png"))?
	});

	Ok(())
}
