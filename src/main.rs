use::clap::Parser;

#[derive(Parser, Debug)]
#[command(name="PokeSpecRS")]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    species: String,

    #[arg(short, long, default_value_t = 1)]
    level: usize,
    
    nickname: Option<String>,
    
    #[arg(short, long, default_value_t = false)]
    shiny: bool,

}
fn main() {
    let args = Args::parse();
}
