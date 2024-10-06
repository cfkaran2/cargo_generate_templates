//! `lib.rs` should contain top-level documentation for the whole crate.
//!
//! `lib.rs` should contain documentation regarding the entire crate.  A common
//! method that users might opt for is to use the [`cargo-readme`][1] tool to
//! create the `README.md` file for the project.  As a result, it is wise to
//! create significant documentation here, enough that if someone is using
//! [`cargo-readme`][1] to create their `README.md`, then the `README.md` makes
//! sense.
//!
//! [1]: https://crates.io/crates/cargo-readme

// This is required to make coverage_helper work right.
//
// HACKER'S NOTE: Don't try to move this inside of the test module, or otherwise
// hide it during non-test builds; it will break your build.
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

pub mod error;

// 'coverage_helper' is only used on tests, and that makes clippy unhappy.  This
//  fixes the issue.
//
// HACKER'S NOTE: Don't try to move this inside of the test module, or otherwise
// hide it during non-test builds; it will break your build.
use coverage_helper as _;

//  ▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄    ▄▄▄▄    ▄▄▄▄▄▄▄▄    ▄▄▄▄
//  ▀▀▀██▀▀▀  ██▀▀▀▀▀▀  ▄█▀▀▀▀█   ▀▀▀██▀▀▀  ▄█▀▀▀▀█
//     ██     ██        ██▄          ██     ██▄
//     ██     ███████    ▀████▄      ██      ▀████▄
//     ██     ██             ▀██     ██          ▀██
//     ██     ██▄▄▄▄▄▄  █▄▄▄▄▄█▀     ██     █▄▄▄▄▄█▀
//     ▀▀     ▀▀▀▀▀▀▀▀   ▀▀▀▀▀       ▀▀      ▀▀▀▀▀

#[cfg(test)]
#[cfg_attr(all(coverage_nightly, test), coverage(off))]
mod tests {
    use bolero::check;

    use super::*;

    #[test]
    fn testing_bolero() {
        check!()
            .with_type::<u32>()
            .for_each(|input| {
               assert!(u128::MAX > (*input).into());
            });
    }
}
