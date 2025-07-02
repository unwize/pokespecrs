pub mod command_logic;
mod console;
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

        #[arg(long)]
        nickname: Option<String>,

        #[arg(short, long, default_value_t = false)]
        shiny: bool,

        #[arg(long, default_value = "PokeSpecRS")]
        ot: String,

        #[arg(long)]
        tid: Option<usize>,

        #[arg(long)]
        sid: Option<usize>,

        #[arg(short, long)]
        gender: Option<String>,

        #[arg(short, long, default_value = Some("poke"))]
        ball: Option<String>,

        #[arg(short, long)]
        nature: Option<String>,

        #[arg(long)]
        ivatk: Option<u16>,
        #[arg(long)]
        ivspatk: Option<u16>,
        #[arg(long)]
        ivdef: Option<u16>,
        #[arg(long)]
        ivspdef: Option<u16>,
        #[arg(long)]
        ivspd: Option<u16>,
        #[arg(long)]
        ivhp: Option<u16>,
        #[arg(long)]
        evatk: Option<u16>,
        #[arg(long)]
        evspatk: Option<u16>,
        #[arg(long)]
        evdef: Option<u16>,
        #[arg(long)]
        evspdef: Option<u16>,
        #[arg(long)]
        evspd: Option<u16>,
        #[arg(long)]
        evhp: Option<u16>,
        #[arg(long)]
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
