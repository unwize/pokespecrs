use colored::Colorize;

pub fn err(msg: &str) {
    println!("{}", msg.red())
}
pub fn info(msg: &str) {
    println!("{}", msg.yellow())
}
pub fn success(msg: &str) {
    println!("{}", msg.green())
}
