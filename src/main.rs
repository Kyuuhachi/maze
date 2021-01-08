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

fn main() -> Result<(), Box<dyn std::error::Error>> {
	use std::time::Instant;
	let time0 = Instant::now();
	// let gen = gen::growing_tree::BackTrack;
	// let gen = gen::growing_tree::PrimSimplified;
	let gen = gen::growing_tree::PrimTrue;
	// let gen = gen::recursive_division::RecursiveDivision;
	// let gen = gen::kruskal::Kruskal;
	// let gen = gen::binary::BinaryTree;
	// let gen = gen::sidewinder::SideWinder;
	let mut rng = rand::rngs::StdRng::from_entropy();
	let maze = gen.generate(&mut rng, (1920, 1080));

	let time1 = Instant::now();
	println!("Generated in {:?}", time1 - time0);
	let img = render::render(&mut rng, &maze, 64, 0., 1.0);
	let time2 = Instant::now();
	println!("Rendered in {:?}", time2 - time1);
	img.save("test.png")?;
	let time3 = Instant::now();
	println!("Saved in {:?}", time3 - time2);
	println!("Total {:?}", time3 - time0);

	Ok(())
}
