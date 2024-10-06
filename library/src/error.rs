//! The common error type used by this crate.
//!
//! This is the common error type returned in the [`Err`] variant by any
//! function or method in this crate.  If it makes sense to do so, the error
//! returned by underlying crates used by this crate will be converted into a
//! variant of the [`Error`] enum and returned to you.

use std::{
    io,
    num::{
        ParseFloatError,
        ParseIntError,
    },
};

use semver::Error as SemverError;
use thiserror;

/// This is the common error type used by this crate.
///
/// All errors that this crate can return are described within this enum.  It
/// uses the [`thiserror::Error`] derive macro to derive [`std::error::Error`].
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Returned when there is an I/O error that came from any of the crates
    /// used by this crate.
    #[error("Received an I/O error.  Error was: {source}")]
    Io {
        /// The source of this error.
        #[from]
        source: io::Error,
    },

    /// Returned when the standard library is unable to parse a string as a
    /// floating point number.
    #[error("There was an error while parsing a string as a floating point \
             number.  Error was {source}.")]
    ParseFloatError {
        /// The error that the parser ran into.
        #[from]
        source: ParseFloatError,
    },

    /// Returned when the standard library is unable to parse a string as a
    /// floating point number.
    #[error("There was an error while parsing a string as an integer.  \
             Error was {source}.")]
    ParseIntError {
        /// The error that the parser ran into.
        #[from]
        source: ParseIntError,
    },

    /// Some types implement [`std::str::FromStr`], which lets you use
    /// [`str::parse`] to create a new instance of the type from a string.
    /// Parsing is always a potentially error-prone affair, and the number of
    /// ways things can go wrong is large.  This error is returned if anything
    /// goes wrong.  The error contains only a little bit of information as it
    /// is impossible to capture all possible issues *a-priori*, so there is a
    /// very simple catch-all string in here, with the expectation that users
    /// will figure out what happened on their own (or with maintainer help).
    #[error("There was an error parsing the string '{input}' as a \
             '{expected_type}'.  The implementation of 'std::str::FromStr' \
             on '{expected_type}' suggests that the error was \
             '{probable_error}', but computers being computers, that could \
             be wrong.  Please read the code and try to figure out what the \
             issue is.  If you can't figure it out (or worse, if it turns \
             out that the implementation of 'std::str::FromStr' is buggy), \
             please file a bug report so that the maintainers can try to \
             fix it.")]
    ParseError {
        /// The string that was being parsed.
        input: String,

        /// The type that the string was being parsed into.
        expected_type: String,

        /// What we think the error was.  **WARNING!** This is an educated
        /// guess, and could be wrong!  You may have some serious debugging to
        /// do if you get this error...
        probable_error: String,
    },

    /// Returned when there is an error parsing version number.
    #[error("Error with parsing a version number.  Error was: {source}")]
    SemVer {
        /// The source of this error.
        #[from]
        source: SemverError,
    },

    /// Returned when the standard library is unable to format some value for
    /// presentation correctly.
    #[error("There was an error while formatting a string.  Error was \
             {source}.")]
    FmtError {
        /// The error that the parser ran into.
        #[from]
        source: core::fmt::Error,
    },
}

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
    #[test]
    fn it_works() {
        assert!(true);
    }
}
