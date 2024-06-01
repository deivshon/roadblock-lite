pub mod config;
pub mod utils;

use std::path::Path;

use clap::Parser;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short = 't', long)]
    target: String,
    #[arg(short = 'c', long, default_value_t = String::from(""))]
    config_path: String,
}

fn main() {
    let args = Args::parse();
    let config_path = if args.config_path.len() > 0 {
        Path::new(&args.config_path).to_path_buf()
    } else {
        match config::path::default_config() {
            Ok(p) => p,
            Err(e) => utils::failure(&format!("could not get default config path: {}", e)),
        }
    };

    println!("{:?}", config_path)
}
