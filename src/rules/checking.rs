use regex::Regex;

use crate::config::models::{Config, Rule};
use crate::rules::models::RuleCheck;

pub fn check_on_config(target: String, config: Config) -> Vec<RuleCheck> {
    let mut results: Vec<RuleCheck> = Vec::new();
    for rule in config.rules {
        results.push(check_single_rule(&target, rule))
    }

    results
}

fn check_single_rule(target: &String, rule: Rule) -> RuleCheck {
    let regex = match Regex::new(&rule.regex) {
        Ok(r) => r,
        Err(e) => {
            return RuleCheck {
                rule,
                matches: Err(e.into()),
            }
        }
    };

    return RuleCheck {
        matches: Ok(regex.is_match(&target)),
        rule,
    };
}
