use::clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(name="PokeSpecRS")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#subcommands
#[derive(Subcommand, Debug)]
enum Commands {
    Generate {
        #[arg(short, long)]
        species: String,

        #[arg(short, long, default_value_t = 1)]
        level: usize,

        nickname: Option<String>,

        #[arg(short, long, default_value_t = false)]
        shiny: bool,
    }
}
fn main() {
    
}
