use figment::{
    Figment,
    providers::{Env, Format, Json},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    cache: bool,
    cache_dir: Option<String>,
}
