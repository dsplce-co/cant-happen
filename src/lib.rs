#![cfg_attr(test, feature(downcast_unchecked))]

pub mod prelude;
mod utils;

#[cfg(test)]
mod tests;

use lazy_static::*;
use std::sync::OnceLock;

lazy_static! {
    static ref REPOSITORY_NEW_ISSUE_URL: OnceLock<String> = OnceLock::new();
}

pub trait CantHappen<T> {
    #[cfg_attr(
        feature = "github",
        doc = "To use with `Result` or `Option` when it's visible in the code that panic is not expected, prints error about reporting GitHub issue"
    )]
    #[cfg_attr(
        not(feature = "github"),
        doc = "To use with `Result` or `Option` when it's visible in the code that panic is not expected"
    )]
    fn cant_happen(self, why: &'static str) -> T;
}

impl<T, U: std::fmt::Debug> CantHappen<T> for Result<T, U> {
    fn cant_happen(self, why: &'static str) -> T {
        let message = utils::cant_happen_fmt(why, &self);
        self.expect(&message)
    }
}

impl<T> CantHappen<T> for Option<T> {
    fn cant_happen(self, why: &'static str) -> T {
        let message = utils::cant_happen_fmt(why, &self.as_ref().ok_or("None"));
        self.expect(&message)
    }
}

pub fn set_repository_new_issue_url(url: &str) {
    let url = url.to_string();

    REPOSITORY_NEW_ISSUE_URL
        .set(url)
        .expect("You can't set the repository new issue URL more than once");
}
