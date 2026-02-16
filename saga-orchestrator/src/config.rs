use std::env;

#[derive(Clone)]
pub struct Config {
    pub payment_url: String,
    pub account_url: String,
    pub ledger_url: String,
    pub audit_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            payment_url: env::var("PAYMENT_URL").unwrap(),
            account_url: env::var("ACCOUNT_URL").unwrap(),
            ledger_url: env::var("LEDGER_URL").unwrap(),
            audit_url: env::var("AUDIT_URL").unwrap(),
        }
    }
}