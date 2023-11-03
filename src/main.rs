use std::path::{PathBuf};
use clap::Parser;

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

    //TODO hier können mit der Zeit weitere args eingebaut werden
}

fn main() {
    let args = Args::parse();
    //args an Einlesefunktion weiter geben
    //algo starten
    //ausgabes

}