pub mod config;
pub mod rules;
pub mod utils;

use std::path::Path;

use clap::Parser;
use config::core::get_config;
use rules::checking::check_on_config;
use utils::{failure, warning};

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
    let config_path = if !args.config_path.is_empty() {
        Path::new(&args.config_path).to_path_buf()
    } else {
        match config::path::default_config() {
            Ok(p) => p,
            Err(e) => failure(&format!("could not get default config path: {}", e)),
        }
    };

    let config = match get_config(&config_path) {
        Ok(c) => c,
        Err(e) => failure(&format!(
            "could not parse config at `{:?}`: {}",
            &config_path, e
        )),
    };

    let check_result = check_on_config(args.target, config);
    for rule_result in check_result {
        let rule_matches = match rule_result.matches {
            Ok(m) => m,
            Err(e) => {
                warning(&format!(
                    "rule {} could not be evaluated: {}",
                    rule_result.rule.name, e
                ));
                continue;
            }
        };

        if rule_matches {
            failure(&format!("rule {} matches", rule_result.rule.name))
        }
    }
}
