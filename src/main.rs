use std::path::{PathBuf};
use clap::Parser;
use crate::input::parse_input;
use crate::output::{Schedule, Solution};
use crate::output::output;

mod input;
mod output;

/// Todoo makespan minimization bliblablub
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path of the input data
    #[arg(short, long)]
    path: PathBuf,

    /// Whether the output should be written in a file or not
    #[arg(short, long, action)]
    write: bool,
    //TODO arg für algorithmus auswahl + für write path
    //hier können mit der Zeit weitere args eingebaut werden
}

fn main() {//TODO bissel logging hinzufügen
    let args = Args::parse();
    let input = parse_input(args.path);
    //println!("{:?}", input);
    //algo starten...
    let s = Solution::new(51, Schedule::new(vec![(3, 0), (2, 44), (1, 0)]));

    output(s, args.write);
}