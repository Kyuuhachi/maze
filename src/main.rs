mod maze;
mod render;
mod gen {
	pub mod growing_tree;
	pub mod recursive_division;
	pub mod kruskal;
	pub mod binary;
	pub mod sidewinder;
	pub mod eller;
}

use rand::prelude::*;
use maze::Generator;
use structopt::StructOpt;
use std::path::PathBuf;
use structopt::clap::AppSettings;

// FIXME: --help doesn't work quite as well as I wish. It says SUBCOMMANDS, which feels
// kinda incorrect, and putting it after a subcommand prints something completely wrong.
#[derive(Debug, StructOpt)]
#[structopt(about,
	setting(AppSettings::ColoredHelp),
	setting(AppSettings::AllArgsOverrideSelf),
	setting(AppSettings::AllowNegativeNumbers),
	setting(AppSettings::DisableHelpSubcommand),
	setting(AppSettings::DeriveDisplayOrder),
	setting(AppSettings::InferSubcommands),
	setting(AppSettings::UnifiedHelpMessage),
	setting(AppSettings::VersionlessSubcommands),
)]
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

	#[structopt(subcommand)]
	mode: Option<Mode>,
}

#[derive(Debug, StructOpt)]
enum Mode {
	Backtrack {
		#[structopt(default_value="1")] turn: f32
	},
	Prim,
	PrimSimplified {
		#[structopt(default_value="1")] turn: f32
	},
	RecursiveDivision,
	Kruskal,
	BinaryTree {
		#[structopt(parse(try_from_str=parse_diag_direction))]
		direction: Option<gen::binary::Direction>,
	},
	Sidewinder,
	Eller,
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

fn parse_diag_direction(s: &str) -> Result<gen::binary::Direction, &'static str> {
	use gen::binary::Direction::*;
	match s {
		"se" | "southeast" => Ok(Southeast),
		"sw" | "southwest" => Ok(Southwest),
		"nw" | "northwest" => Ok(Northwest),
		"ne" | "northeast" => Ok(Northeast),
		_ => Err("must be one of 'se', 'sw', 'nw', or 'ne'")
	}
}

fn get_generator(rng: &mut StdRng, mode: Option<Mode>) -> Box<dyn Generator> {
	let mode = mode.unwrap_or_else(|| {
		match rng.gen_range(0..7) {
			0 => Mode::Backtrack{turn:1.},
			1 => Mode::Prim,
			2 => Mode::PrimSimplified{turn:1.},
			3 => Mode::RecursiveDivision,
			4 => Mode::Kruskal,
			5 => Mode::BinaryTree{direction:None},
			6 => Mode::Sidewinder,
			7 => Mode::Eller,
			_ => panic!(),
		}
	});

	match mode {
		Mode::Backtrack{turn}       => Box::new(gen::growing_tree::Backtrack(turn)),
		Mode::Prim                  => Box::new(gen::growing_tree::PrimTrue),
		Mode::PrimSimplified{turn}  => Box::new(gen::growing_tree::PrimSimplified(turn)),
		Mode::RecursiveDivision     => Box::new(gen::recursive_division::RecursiveDivision),
		Mode::Kruskal               => Box::new(gen::kruskal::Kruskal),
		Mode::BinaryTree{direction} => Box::new(gen::binary::BinaryTree(direction.unwrap_or_else(|| {
			use gen::binary::Direction::*;
			*vec![Southeast, Southwest, Northwest, Northeast].choose(rng).unwrap()
		}))),
		Mode::Sidewinder            => Box::new(gen::sidewinder::Sidewinder),
		Mode::Eller                 => Box::new(gen::eller::Eller),
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let opt = Args::from_args();

	let seed = opt.seed.unwrap_or_else(|| random());
	if !opt.quiet { println!("Seed: {}", seed); }
	let mut rng = StdRng::seed_from_u64(seed);
	let gen = get_generator(&mut rng, opt.mode);

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
