#[macro_use]
extern crate num_derive;
mod api;
mod cache;
pub mod command_logic;
mod console;
pub mod enums;
mod errors;
pub mod spec;
mod util;

use crate::command_logic::CommandLogic;
use clap::Subcommand;
use clap::{Args, Parser};
use figment::providers::Format;
use miette::{IntoDiagnostic, Result};

#[derive(Parser, Debug)]
#[command(name = "PokeSpecRS")]
#[command(version, about, long_about = None)]
struct Cli {
    // All commands are held within the Commands enum
    #[command(subcommand)]
    command: Commands,
}

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    // The main command. Used to generate Pokemon specs.
    Generate {
        // Required, positional
        species: String,

        // Optional, flag-based (with --)
        #[arg(short, long)]
        ability: Option<String>,
        #[arg(short, long, default_value_t = 1)]
        level: u8,
        #[arg(long, alias = "nick")]
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
        #[arg(short, long, alias="ba", default_value = "pokeball")]
        ball: String,
        #[arg(short, long, alias = "nat")]
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
        #[arg(long, num_args = 1..4)]
        moveset: Vec<String>,
        #[arg(long = "gen")]
        generation: Option<u8>,
    },

    // The Cache command and its various subcommands.
    // See: https://github.com/clap-rs/clap/blob/3ef784b516b2c9fbf6adb1c3603261b085561be7/examples/git-derive.rs
    Cache(CacheArgs),
}

#[derive(Debug, Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
struct CacheArgs {
    // A struct that hosts the Cache command's sub-commands.
    #[command(subcommand)]
    command: CacheCommands,
}

#[derive(Debug, Subcommand, Clone)]
enum CacheCommands {
    Clear {},
    Check { species: String },
    Purge { species: String },
    Validate {},
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("{:?}", args.command);

    match &args.command {
        Commands::Generate { .. } => command_logic::Generate.execute(args.command),
        Commands::Cache { .. } => command_logic::Cache.execute(args.command),
    }
}
