pub mod enums;
mod generate;
mod spec;

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
        species: String,

        #[arg(short, long)]
        ability: Option<String>,

        #[arg(short, long, default_value_t = 1)]
        level: usize,

        nickname: Option<String>,

        #[arg(short, long, default_value_t = false)]
        shiny: bool,

        #[arg(long, default_value = "PokeSpecRS")]
        ot: String,
        tid: Option<usize>,
        sid: Option<usize>,

        #[arg(short, long)]
        gender: Option<String>,

        #[arg(short, long)]
        ball: Option<String>,

        #[arg(short, long)]
        nature: Option<String>,

        ivatk: Option<u8>,
        ivspatk: Option<u8>,
        ivdef: Option<u8>,
        ivspdef: Option<u8>,
        ivspd: Option<u8>,
        ivhp: Option<u8>,

        evatk: Option<u8>,
        evspatk: Option<u8>,
        evdef: Option<u8>,
        evspdef: Option<u8>,
        evspd: Option<u8>,
        evhp: Option<u8>,

        moveset: Option<Vec<String>>,
    }
}
fn main() {
    let args = Cli::parse();

    println!("{:?}", args);
}
