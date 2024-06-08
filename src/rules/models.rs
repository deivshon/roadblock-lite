use anyhow::Result;

use crate::config::models::Rule;

pub struct RuleCheck {
    pub matches: Result<bool>,
    pub rule: Rule,
}
