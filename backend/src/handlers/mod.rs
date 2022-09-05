pub mod account;
pub mod authentication;
pub mod index;
pub mod rssfeed;
pub mod error;

pub enum ResponseError {
    Success = 0,
    Failure = 1,
}

pub enum AuthProvider {
    Plumage = 0,
    Google = 1,
}

#[cfg(test)]
mod account_test;
#[cfg(test)]
mod rssfeed_test;
