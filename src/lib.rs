//! A collection of utilities for working with Rust's Option type.
//!
//! ## Example
//!
//! ```
//! use option_utils::OptionUtils;
//!
//! let mut x = Some("Hello world".to_owned());
//! x.inner_mut(|s| s.push('!'));
//! assert_eq!(x, Some("Hello world!".to_owned()));
//!
//! let path = Some("dir");
//! let path: Option<std::path::PathBuf> = path.map_into();
//! assert_eq!(path, Some(std::path::Path::new("dir").to_owned()));
//!
//! let num = Some(10_u64);
//! let num: Option<u8> = num.try_map_into().unwrap();
//! assert_eq!(num, Some(10_u8));
//! ```
//!
//! ## License
//!
//! This project is licensed under the Apache-2.0 license.
//!

/// A collection of utilities for working with Rust's Option type.
pub trait OptionUtils<T> {
    /// Run a function on the inner value of an Option.
    fn inner_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T);

    /// Map an Option into another type.
    fn map_into<R>(self) -> Option<R>
    where
        Self: Sized,
        T: Into<R>;

    /// Map an Option into another type, returning an error if the conversion fails.
    fn try_map_into<R, E>(self) -> Result<Option<R>, E>
    where
        Self: Sized,
        T: TryInto<R, Error = E>;
}

impl<T> OptionUtils<T> for Option<T> {
    fn inner_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        if let Some(inner) = self.as_mut() {
            f(inner);
        }
    }

    fn try_map_into<R, E>(self) -> Result<Option<R>, E>
    where
        Self: Sized,
        T: TryInto<R, Error = E>,
    {
        self.map(TryInto::try_into).transpose()
    }

    fn map_into<R>(self) -> Option<R>
    where
        Self: Sized,
        T: Into<R>,
    {
        self.map(Into::into)
    }
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_inner_mut() -> Result<()> {
        let mut x = Some("Hello world".to_owned());
        x.inner_mut(|s| s.push('!'));
        assert_eq!(x, Some("Hello world!".to_owned()));
        Ok(())
    }

    #[test]
    fn test_map_into() -> Result<()> {
        let path = Some("hey");
        let path: Option<PathBuf> = path.map_into();
        assert_eq!(path, Some(Path::new("hey").to_owned()));
        Ok(())
    }

    #[test]
    fn test_map_try_into() -> Result<()> {
        let num = Some(10_u64);
        let num: Option<u8> = num.try_map_into()?;
        assert_eq!(num, Some(10_u8));
        Ok(())
    }
}
