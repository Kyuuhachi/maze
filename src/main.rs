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
use structopt::StructOpt;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Args {
	/// Seed to use for the RNG.
	///
	/// If unspecified, choose a random seed.
	#[structopt(short, long)] seed: Option<u64>,
	
	/// The output path to write to.
	#[structopt(short, long, default_value="out.png")] outfile: PathBuf,

	/// The color range to use on the resulting image, in degrees.
	///
	/// There are three supported formats:
	///   • intervals: 0..360,
	///   • base+offset: 180+60 = 120..240 (can also use unicode ±),
	///   • a single number: 60 = 60+30 = 30..90.
	///
	/// The interval is divided into <nregions> equally sized intervals, and the colors are taken
	/// from the middle of each interval. As a consequence, if there is an even number of regions,
	/// the center point will not be included.
	#[structopt(short, long,
		verbatim_doc_comment,
		parse(try_from_str=parse_hue),
		default_value="0..360",
	)] color: (f32, f32),

	/// Print timing information.
	#[structopt(short, long)] verbose: bool,

	/// Do not print seed.
	#[structopt(short, long, conflicts_with="verbose")] quiet: bool,

	/// Width of the image.
	width: usize,

	/// Height of the image.
	height: usize,

	/// How many regions to use for coloring.
	nregions: u32,
}

fn parse_hue(s: &str) -> Result<(f32, f32), std::num::ParseFloatError> {
	macro_rules! try_split {
		($x:literal, $y:expr) => {
			let xs: Vec<&str> = s.split($x).collect();
			if xs.len() == 2 {
				let a: f32 = xs[0].parse()?;
				let b: f32 = xs[1].parse()?;
				return Ok($y(a, b));
			}
		};
	}

	try_split!("..", |a, b| (a, b));
	try_split!("+", |a, b| (a+b, a-b));
	try_split!("±", |a, b| (a+b, a-b));
	s.parse().map(|a: f32| (a-30., a+30.))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let opt = Args::from_args();

	let gen = gen::growing_tree::BackTrack;
	// let gen = gen::growing_tree::PrimSimplified;
	// let gen = gen::growing_tree::PrimTrue;
	// let gen = gen::recursive_division::RecursiveDivision;
	// let gen = gen::kruskal::Kruskal;
	// let gen = gen::binary::BinaryTree;
	// let gen = gen::sidewinder::SideWinder;

	let seed = opt.seed.unwrap_or_else(|| rand::random());
	if !opt.quiet { println!("Seed: {}", seed); }
	let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

	macro_rules! time {
		($x:literal, $y:expr) => { {
			use std::time::Instant;
			let time1 = Instant::now();
			let v = $y;
			let time2 = Instant::now();
			if opt.verbose { println!("{:10} {:?}", $x, time2 - time1); }
			v
		} };
	}

	time!("Total", {
		let maze = time!("Generation", gen.generate(&mut rng, (opt.width, opt.height)));
		let img = time!("Rendering", render::render(&mut rng, &maze, opt.nregions, opt.color));
		time!("Saving", img.save_with_format(opt.outfile, image::ImageFormat::Png))?
	});

	Ok(())
}
