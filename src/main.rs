mod maze;
mod render;
mod gen {
	pub mod growing_tree;
	pub mod recursive_division;
	pub mod kruskal;
}

fn main() {
	use std::time::Instant;
	let time0 = Instant::now();
	// let maze = gen::recursive_division::generate(300, 300);
	// let maze = gen::growing_tree::prim_simplified(300, 300);
	// let maze = gen::growing_tree::prim_true(300, 300);
	// let maze = gen::growing_tree::backtrack(300, 300);
	let maze = gen::growing_tree::prim_true(1920, 1080);
	// let maze = gen::kruskal::generate(1920, 1080);
	// let maze = gen::kruskal::generate(60, 30);

	let time1 = Instant::now();
	println!("Generated in {:?}", time1 - time0);
	let img = render::render(&maze, 64, 0., 1.0);
	let time2 = Instant::now();
	println!("Rendered in {:?}", time2 - time1);
	img.save("test.png").unwrap();
	let time3 = Instant::now();
	println!("Saved in {:?}", time3 - time2);
	println!("Total {:?}", time3 - time0);
}
