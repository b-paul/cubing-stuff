use clap::Parser;

// TODO allow specifying the axes and slice count and flipped edge count ? of output drs maybe. An
// interface would need to be made

/// A scramble to dr-Xs solver
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The scramble you wish to solve written in HTM. Niss brackets are allowed :)
    scramble: String,

    /// Print solutions with final qts reversed
    #[arg(short)]
    all: bool,

    /// Print only the number of solutions
    #[arg(short)]
    count: bool,

    /// Minimum solution length
    #[arg(short)]
    min: Option<usize>,

    /// Maximum solution length
    #[arg(short('M'))]
    max: Option<usize>,

    /// Find at most N solutions
    #[arg(short)]
    num: Option<usize>,

    /// List all solutions of optimal length
    #[arg(short)]
    opt: bool,

    /// List all solutions of length optimal + N
    #[arg(short('O'), id("N"))]
    maxfrom: Option<usize>,

    /// Use niss
    #[arg(short('N'))]
    niss: bool,

    /// Do not print the number of moves
    #[arg(short)]
    plain: bool,
}

fn main() {
    let _args = Args::parse();

    println!("Sorry I haven't actually made the solver yet...");
}
