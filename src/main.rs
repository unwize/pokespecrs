pub mod command_logic;
pub mod enums;
pub mod generate;
pub mod spec;

use crate::command_logic::CommandLogic;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(name = "PokeSpecRS")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    Generate {
        species: String,

        #[arg(short, long)]
        ability: Option<String>,

        #[arg(short, long, default_value_t = 1)]
        level: u8,

        nickname: Option<String>,

        #[arg(short, long, default_value_t = false)]
        shiny: bool,

        #[arg(long, default_value = "PokeSpecRS")]
        ot: String,
        tid: Option<usize>,
        sid: Option<usize>,

        #[arg(short, long)]
        gender: Option<String>,

        #[arg(short, long, default_value = "poke")]
        ball: Option<String>,

        #[arg(short, long)]
        nature: Option<String>,

        ivatk: Option<u16>,
        ivspatk: Option<u16>,
        ivdef: Option<u16>,
        ivspdef: Option<u16>,
        ivspd: Option<u16>,
        ivhp: Option<u16>,

        evatk: Option<u16>,
        evspatk: Option<u16>,
        evdef: Option<u16>,
        evspdef: Option<u16>,
        evspd: Option<u16>,
        evhp: Option<u16>,

        moveset: Option<Vec<String>>,
    },
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let args = Cli::parse();

    println!("{:?}", args.command);

    match &args.command {
        Commands::Generate { .. } => {
            command_logic::Generate.execute(args.command);
        }
    }
}
