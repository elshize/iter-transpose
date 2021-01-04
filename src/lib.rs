//! This crate provides the [`IterTranspose`] trait that turns an `Option<I: IntoIterator>`
//! (or `Result<I: IntoIterator, E>`) into an iterator of `Option<I::Item>` (or `Result<I::Item, E>`):
//!
//! ```
//! use iter_transpose::IterTranspose;
//!
//! assert_eq!(
//!     Some(vec![1, 2, 3]).transpose_into_iter().collect::<Vec<_>>(),
//!     vec![Some(1), Some(2), Some(3)],
//! );
//!
//! assert_eq!(
//!     Result::<Vec<i32>, ()>::Ok(vec![1, 2, 3])
//!         .transpose_into_iter()
//!         .collect::<Vec<_>>(),
//!     vec![Result::<i32, ()>::Ok(1), Ok(2), Ok(3)]
//! );
//! ```
//!
//! **Note:** if the value is either `None` or `Err`, the iterator will be **infinite**.
//! You can use [`take_while_some`][`OptionTransposedIter::take_while_some`]
//! or [`take_while_ok`][`ResultTransposedIter::take_while_ok`] to truncate them.
//!
//!
//! ```
//! use iter_transpose::IterTranspose;
//!
//! assert_eq!(
//!     Option::<Vec<i32>>::None
//!         .transpose_into_iter()
//!         .take(5)                // We can take as many as we want.
//!         .collect::<Vec<_>>(),
//!     vec![None, None, None, None, None],
//! );
//! assert_eq!(
//!     Result::<Vec<i32>, ()>::Err(())
//!         .transpose_into_iter()
//!         .take(5)                // We can take as many as we want.
//!         .collect::<Vec<_>>(),
//!     vec![Result::<i32, ()>::Err(()), Err(()), Err(()), Err(()), Err(())],
//! );
//! ```
//!
//! Note that in case of `Result<T, E>`, it must hold that `E: Clone + std::fmt::Debug`.
//!
//! # Use Case
//!
//! The main use case is when there is some optional data loaded separately from the required
//! data, e.g., from another file or other source, and we want to produce either a value or
//! `None` for each element from the required list, depending on whether the optional data was
//! loaded or not.
//!
//! ```
//! # use iter_transpose::IterTranspose;
//! #[derive(Debug, PartialEq, Eq)]
//! struct Item {
//!     name: &'static str,
//!     description: Option<&'static str>,
//! }
//!
//! fn items(names: Vec<&'static str>, descriptions: Option<Vec<&'static str>>) -> Vec<Item> {
//!     names
//!         .into_iter()
//!         .zip(descriptions.transpose_into_iter())
//!         .map(|(name, description)| Item { name, description })
//!         .collect()
//! }
//!
//! # fn main() {
//! assert_eq!(
//!     items(vec!["Alice", "Bob", "Charlie"], None),
//!     vec![
//!         Item {
//!             name: "Alice",
//!             description: None,
//!         },
//!         Item {
//!             name: "Bob",
//!             description: None,
//!         },
//!         Item {
//!             name: "Charlie",
//!             description: None,
//!         },
//!     ]
//! );
//!
//! assert_eq!(
//!     items(
//!         vec!["Alice", "Bob", "Charlie"],
//!         Some(vec!["in Wonderland", "the builder", "likes chocolate"])
//!     ),
//!     vec![
//!         Item {
//!             name: "Alice",
//!             description: Some("in Wonderland"),
//!         },
//!         Item {
//!             name: "Bob",
//!             description: Some("the builder"),
//!         },
//!         Item {
//!             name: "Charlie",
//!             description: Some("likes chocolate"),
//!         },
//!     ]
//! );
//! # }
//! ```
//!
//! # Other Examples
//!
//! You can also use this function to iterate over all existing elements.
//! Handy functions are [`unwrap_while_some`][`OptionTransposedIter::unwrap_while_some`]
//! for [`Option`] and [`unwrap_while_ok`][`ResultTransposedIter::unwrap_while_ok`] for [`Result`]:
//!
//! ```
//! # use iter_transpose::IterTranspose;
//! assert_eq!(
//!     Some((0..5))
//!         .transpose_into_iter()
//!         .unwrap_while_some()
//!         .collect::<Vec<_>>(),
//!     Some((0..5))
//!         .into_iter()
//!         .flatten()
//!         .collect::<Vec<_>>(),
//! );
//! ```
//!
//! [`Option::transpose`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose
//! [`Result`]: https://doc.rust-lang.org/stable/std/result/enum.Result.html

#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::default_trait_access,
    clippy::inline_always
)]

/// Provides [`transpose_into_iter`][`IterTranspose::transpose_into_iter`]
/// function for the implementing structs.
///
/// This trait is implemented for both [`Option`] and [`Result`], and it functions as an
/// extension of the API of these two structs. See the [crate-level documentation](index.html)
/// for more information and examples.
///
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
/// [`Result`]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
pub trait IterTranspose {
    /// The iterator type produced by [`transpose_into_iter`][`IterTranspose::transpose_into_iter`].
    type Iter: Iterator;

    /// If called on an option containing a collection or an iterator, it produces an iterator
    /// of options, and similarly for results.
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_transpose::IterTranspose;
    /// assert_eq!(
    ///     Some(vec![1, 2, 3]).transpose_into_iter().take(3).collect::<Vec<_>>(),
    ///     vec![Some(1), Some(2), Some(3)],
    /// );
    /// assert_eq!(
    ///     Result::<Vec<i32>, ()>::Ok(vec![1, 2, 3]).transpose_into_iter().take(3).collect::<Vec<_>>(),
    ///     vec![Result::<i32, ()>::Ok(1), Ok(2), Ok(3)],
    /// );
    /// assert_eq!(
    ///     Option::<Vec<i32>>::None.transpose_into_iter().take(3).collect::<Vec<_>>(),
    ///     vec![None, None, None],
    /// );
    /// assert_eq!(
    ///     Result::<Vec<i32>, ()>::Err(()).transpose_into_iter().take(3).collect::<Vec<_>>(),
    ///     vec![Result::<i32, ()>::Err(()), Err(()), Err(())],
    /// );
    /// ```
    fn transpose_into_iter(self) -> Self::Iter;
}

impl<I> IterTranspose for Option<I>
where
    I: IntoIterator,
{
    type Iter = OptionTransposedIter<I::IntoIter>;

    fn transpose_into_iter(self) -> Self::Iter {
        OptionTransposedIter {
            inner: self.map(I::into_iter),
        }
    }
}

impl<I, E> IterTranspose for Result<I, E>
where
    I: IntoIterator,
    E: Clone + std::fmt::Debug,
{
    type Iter = ResultTransposedIter<I::IntoIter, E>;

    fn transpose_into_iter(self) -> Self::Iter {
        ResultTransposedIter {
            inner: self.map(I::into_iter),
        }
    }
}

/// Result of calling [`IterTranspose::transpose_into_iter`] on [`Option`].
///
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
pub struct OptionTransposedIter<I> {
    inner: Option<I>,
}

impl<I> OptionTransposedIter<I>
where
    I: Iterator,
{
    /// Returns an iterator adapter that takes elements while they are `Some`;
    /// shorthand for `take_while(Option::is_some)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use iter_transpose::IterTranspose;
    /// assert_eq!(
    ///     Some(vec![1, 2]).transpose_into_iter().take_while_some().collect::<Vec<_>>(),
    ///     vec![Some(1), Some(2)],
    /// );
    /// ```
    pub fn take_while_some(self) -> impl Iterator<Item = <Self as Iterator>::Item> {
        self.take_while(Option::is_some)
    }

    /// Returns an iterator adapter that takes elements while they are `Some`, and unwraps them.
    ///
    /// # Example
    ///
    /// ```
    /// # use iter_transpose::IterTranspose;
    /// assert_eq!(
    ///     Some(vec![1, 2]).transpose_into_iter().unwrap_while_some().collect::<Vec<_>>(),
    ///     vec![1, 2],
    /// );
    /// ```
    pub fn unwrap_while_some(self) -> impl Iterator<Item = I::Item> {
        self.take_while(Option::is_some).map(Option::unwrap)
    }
}

impl<I> Iterator for OptionTransposedIter<I>
where
    I: Iterator,
{
    type Item = Option<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .as_mut()
            .map_or(Some(None), |iter| iter.next().map(Some))
    }
}

/// Result of calling [`IterTranspose::transpose_into_iter`] on [`Result`].
///
/// [`Result`]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
pub struct ResultTransposedIter<I, E> {
    inner: Result<I, E>,
}

impl<I, E> ResultTransposedIter<I, E>
where
    I: Iterator,
    E: Clone + std::fmt::Debug,
{
    /// Returns an iterator adapter that takes elements while they are `Some`;
    /// shorthand for `take_while(Option::is_some)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use iter_transpose::IterTranspose;
    /// assert_eq!(
    ///     Some(vec![1, 2]).transpose_into_iter().take_while_some().collect::<Vec<_>>(),
    ///     vec![Some(1), Some(2)],
    /// );
    /// ```
    pub fn take_while_ok(self) -> impl Iterator<Item = <Self as Iterator>::Item> {
        self.take_while(Result::is_ok)
    }

    /// Returns an iterator adapter that takes elements while they are `Some`, and unwraps them.
    ///
    /// # Example
    ///
    /// ```
    /// # use iter_transpose::IterTranspose;
    /// assert_eq!(
    ///     Result::<Vec<i32>, ()>::Ok(vec![1, 2])
    ///         .transpose_into_iter()
    ///         .unwrap_while_ok()
    ///         .collect::<Vec<_>>(),
    ///     vec![1, 2],
    /// );
    /// ```
    pub fn unwrap_while_ok(self) -> impl Iterator<Item = I::Item> {
        self.take_while(Result::is_ok).map(Result::unwrap)
    }
}

impl<I, E> Iterator for ResultTransposedIter<I, E>
where
    I: Iterator,
    E: Clone,
{
    type Item = Result<I::Item, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.as_mut() {
            Ok(iter) => iter.next().map(Ok),
            Err(err) => Some(Err(err.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_none() {
        use IterTranspose;
        let vec: Option<Vec<i32>> = None;
        assert_eq!(
            vec.transpose_into_iter().take(3).collect::<Vec<_>>(),
            vec![None, None, None]
        );
    }

    #[test]
    fn test_transpose_some() {
        use IterTranspose;
        let vec: Option<Vec<i32>> = Some(vec![1, 2, 3]);
        let iter = vec.map(|v| v.into_iter());
        assert_eq!(
            iter.transpose_into_iter()
                .take_while(Option::is_some)
                .collect::<Vec<_>>(),
            vec![Some(1), Some(2), Some(3)]
        );
    }

    #[test]
    fn test_transpose_none_ref() {
        use IterTranspose;
        let vec: Option<Vec<i32>> = None;
        let iter = vec.as_ref().map(|v| v.iter());
        assert_eq!(
            iter.transpose_into_iter().take(3).collect::<Vec<_>>(),
            vec![None, None, None]
        );
    }

    #[test]
    fn test_transpose_some_ref() {
        use IterTranspose;
        let vec: Option<Vec<i32>> = Some(vec![1, 2, 3]);
        assert_eq!(
            vec.as_ref()
                .transpose_into_iter()
                .take_while(Option::is_some)
                .collect::<Vec<_>>(),
            vec![Some(&1), Some(&2), Some(&3)]
        );
    }
}
