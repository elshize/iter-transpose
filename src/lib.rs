//! This crate provides [`IntoTransposedIter`] trait that turns an `Option<I: Iterator>`
//! into an iterator of options.

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

/// Provides [`IntoTransposedIter::into_transposed_iter`] function.
pub trait IntoTransposedIter {
    /// Type of underlying iterator.
    type Iter;

    /// Turns an `Option<T: IntoIterator>` into an iterator of options.
    /// This works somewhat similar to [`Option::transpose`] but for an iterable types instead of
    /// [`Result`].
    ///
    /// Notice that the type of the item of the resulting iterator is
    /// `Option<<T as IntoIterator>::Item>`, not to confuse with the return type of the
    /// [`Iterator::next`] method, which will be `Option<Option<<T as IntoIterator>::Item>>`.
    /// Note that the resulting iterator is always **infinite**.
    /// If executed on `Some(v)`, then the iterator will first yield the elements of `v`
    /// wrapped by `Some`, following by infinite number of `None`.
    /// In particular, the following two are equivalent:
    ///
    /// ```
    /// # use iter_transpose::IntoTransposedIter;
    /// let mut some_iter = Some(Vec::<i32>::new()).into_transposed_iter();
    /// let mut none_iter = Option::<Vec<i32>>::None.into_transposed_iter();
    /// assert_eq!(some_iter.next(), Some(None));
    /// assert_eq!(none_iter.next(), Some(None));
    /// ```
    ///
    /// The main use case is when there is some optional data loaded separately from the required
    /// data, e.g., from another file or other source, and we want to produce either a value or
    /// `None` for each element from the required list, depending on whether the optional data was
    /// loaded or not. (See the example below.)
    ///
    /// In order to only produce as many items as there are elements in the underlying iterator,
    /// one of two associated functions on the resulting transposed iterator can be called:
    /// [`TransposedIter::take_while_some`] or [`TransposedIter::unwrap_while_some`].
    ///
    /// The first simply produces elements while they are `Some`:
    ///
    /// ```
    /// # use iter_transpose::IntoTransposedIter;
    /// assert_eq!(
    ///     Some(vec![1, 2]).into_transposed_iter().take_while_some().collect::<Vec<_>>(),
    ///     vec![Some(1), Some(2)],
    /// );
    /// ```
    ///
    /// The second function additionally unwraps these elements:
    ///
    /// ```
    /// # use iter_transpose::IntoTransposedIter;
    /// assert_eq!(
    ///     Some(vec![1, 2]).into_transposed_iter().unwrap_while_some().collect::<Vec<_>>(),
    ///     vec![1, 2],
    /// );
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use iter_transpose::IntoTransposedIter;
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Item {
    ///     name: &'static str,
    ///     description: Option<&'static str>,
    /// }
    ///
    /// fn items(names: Vec<&'static str>, descriptions: Option<Vec<&'static str>>) -> Vec<Item> {
    ///     names
    ///         .into_iter()
    ///         .zip(descriptions.into_transposed_iter())
    ///         .map(|(name, description)| Item { name, description })
    ///         .collect()
    /// }
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     items(vec!["Alice", "Bob", "Charlie"], None),
    ///     vec![
    ///         Item {
    ///             name: "Alice",
    ///             description: None,
    ///         },
    ///         Item {
    ///             name: "Bob",
    ///             description: None,
    ///         },
    ///         Item {
    ///             name: "Charlie",
    ///             description: None,
    ///         },
    ///     ]
    /// );
    ///
    /// assert_eq!(
    ///     items(
    ///         vec!["Alice", "Bob", "Charlie"],
    ///         Some(vec!["in Wonderland", "the builder", "likes chocolate"])
    ///     ),
    ///     vec![
    ///         Item {
    ///             name: "Alice",
    ///             description: Some("in Wonderland"),
    ///         },
    ///         Item {
    ///             name: "Bob",
    ///             description: Some("the builder"),
    ///         },
    ///         Item {
    ///             name: "Charlie",
    ///             description: Some("likes chocolate"),
    ///         },
    ///     ]
    /// );
    /// # }
    /// ```
    ///
    /// You can also use this function to iterate over all existing elements.
    /// These two are equivalent:
    ///
    /// ```
    /// # use iter_transpose::IntoTransposedIter;
    /// assert_eq!(
    ///     Some((0..5))
    ///         .into_transposed_iter()
    ///         .unwrap_while_some()
    ///         .collect::<Vec<_>>(),
    ///     Some((0..5))
    ///         .into_iter()
    ///         .flatten()
    ///         .collect::<Vec<_>>(),
    /// );
    /// ```
    ///
    /// [`Option::transpose`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose
    /// [`Result`]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
    fn into_transposed_iter(self) -> TransposedIter<Self::Iter>;
}

/// A free function version of [`IntoTransposedIter::into_transposed_iter`].
pub fn transposed_iter<T: IntoIterator>(t: Option<T>) -> TransposedIter<T::IntoIter> {
    TransposedIter {
        optional_iter: t.map(T::into_iter),
    }
}

impl<T: IntoIterator> IntoTransposedIter for Option<T> {
    type Iter = T::IntoIter;

    fn into_transposed_iter(self) -> TransposedIter<Self::Iter> {
        TransposedIter {
            optional_iter: self.map(T::into_iter),
        }
    }
}

/// Result of calling [`IntoTransposedIter::into_transposed_iter`].
pub struct TransposedIter<I> {
    optional_iter: Option<I>,
}

impl<I> TransposedIter<I>
where
    I: Iterator,
{
    /// Returns an iterator adapter that takes elements while they are `Some`;
    /// shorthand for `take_while(Option::is_some)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use iter_transpose::IntoTransposedIter;
    /// assert_eq!(
    ///     Some(vec![1, 2]).into_transposed_iter().take_while_some().collect::<Vec<_>>(),
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
    /// # use iter_transpose::IntoTransposedIter;
    /// assert_eq!(
    ///     Some(vec![1, 2]).into_transposed_iter().unwrap_while_some().collect::<Vec<_>>(),
    ///     vec![1, 2],
    /// );
    /// ```
    pub fn unwrap_while_some(self) -> impl Iterator<Item = I::Item> {
        self.take_while(Option::is_some).map(Option::unwrap)
    }
}

impl<I> Iterator for TransposedIter<I>
where
    I: Iterator,
{
    type Item = Option<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.optional_iter
            .as_mut()
            .map_or(Some(None), |iter| Some(iter.next()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_none() {
        use IntoTransposedIter;
        let vec: Option<Vec<i32>> = None;
        assert_eq!(
            vec.into_transposed_iter().take(3).collect::<Vec<_>>(),
            vec![None, None, None]
        );
    }

    #[test]
    fn test_transpose_some() {
        use IntoTransposedIter;
        let vec: Option<Vec<i32>> = Some(vec![1, 2, 3]);
        let iter = vec.map(|v| v.into_iter());
        assert_eq!(
            iter.into_transposed_iter()
                .take_while(Option::is_some)
                .collect::<Vec<_>>(),
            vec![Some(1), Some(2), Some(3)]
        );
    }

    #[test]
    fn test_transpose_none_ref() {
        use IntoTransposedIter;
        let vec: Option<Vec<i32>> = None;
        let iter = vec.as_ref().map(|v| v.iter());
        assert_eq!(
            iter.into_transposed_iter().take(3).collect::<Vec<_>>(),
            vec![None, None, None]
        );
    }

    #[test]
    fn test_transpose_some_ref() {
        use IntoTransposedIter;
        let vec: Option<Vec<i32>> = Some(vec![1, 2, 3]);
        assert_eq!(
            vec.as_ref()
                .into_transposed_iter()
                .take_while(Option::is_some)
                .collect::<Vec<_>>(),
            vec![Some(&1), Some(&2), Some(&3)]
        );
    }
}
