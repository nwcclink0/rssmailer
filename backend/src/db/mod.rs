pub mod account;
pub mod rssfeed;

mod account_test;
#[cfg(test)]
mod rssfeed_test;

pub static K_RSSMAILER_DB: &str = dotenv!("DATABASE_URL");
