mod maze;
mod render;
mod gen {
	pub mod growing_tree;
	pub mod recursive_division;
	pub mod kruskal;
	pub mod binary;
	pub mod sidewinder;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	use std::time::Instant;
	let time0 = Instant::now();
	// let f = gen::recursive_division::generate;
	// let f = gen::growing_tree::prim_simplified;
	// let f = gen::growing_tree::prim_true;
	// let f = gen::growing_tree::backtrack;
	let f = gen::growing_tree::prim_true;
	// let f = gen::kruskal::generate;
	// let f = gen::binary::generate;
	// let f = gen::sidewinder::generate;
	let maze = f((1920, 1080));

	let time1 = Instant::now();
	println!("Generated in {:?}", time1 - time0);
	let img = render::render(&maze, 64, 0., 1.0);
	let time2 = Instant::now();
	println!("Rendered in {:?}", time2 - time1);
	img.save("test.png")?;
	let time3 = Instant::now();
	println!("Saved in {:?}", time3 - time2);
	println!("Total {:?}", time3 - time0);

	Ok(())
}
