This crate provides `IntoTransposedIter` trait that turns an `Option<I: Iterator>`
into an iterator of options.

# API Documentation

https://elshize.github.io/iter-transpose/

# Overview

Notice that the type of the item of the resulting iterator is
`Option<<T as IntoIterator>::Item>`, not to confuse with the return type of the
`Iterator::next` method, which will be `Option<Option<<T as IntoIterator>::Item>>`.
Note that the resulting iterator is always **infinite**.
If executed on `Some(v)`, then the iterator will first yield the elements of `v`
wrapped by `Some`, following by infinite number of `None`.
In particular, the following two are equivalent:

```
use iter_transpose::IntoTransposedIter;
let mut some_iter = Some(Vec::<i32>::new()).into_transposed_iter();
let mut none_iter = Option::<Vec<i32>>::None.into_transposed_iter();
assert_eq!(some_iter.next(), Some(None));
assert_eq!(none_iter.next(), Some(None));
```

The main use case is when there is some optional data loaded separately from the required
data, e.g., from another file or other source, and we want to produce either a value or
`None` for each element from the required list, depending on whether the optional data was
loaded or not. (See the example below.)

In order to only produce as many items as there are elements in the underlying iterator,
one of two associated functions on the resulting transposed iterator can be called:
`TransposedIter::take_while_some` or `TransposedIter::unwrap_while_some`.

The first simply produces elements while they are `Some`:

```
use iter_transpose::IntoTransposedIter;
assert_eq!(
    Some(vec![1, 2]).into_transposed_iter().take_while_some().collect::<Vec<_>>(),
    vec![Some(1), Some(2)],
);
```

The second function additionally unwraps these elements:

```
use iter_transpose::IntoTransposedIter;
assert_eq!(
    Some(vec![1, 2]).into_transposed_iter().unwrap_while_some().collect::<Vec<_>>(),
    vec![1, 2],
);
```

# Examples

```
use iter_transpose::IntoTransposedIter;

#[derive(Debug, PartialEq, Eq)]
struct Item {
    name: &'static str,
    description: Option<&'static str>,
}

fn items(names: Vec<&'static str>, descriptions: Option<Vec<&'static str>>) -> Vec<Item> {
    names
        .into_iter()
        .zip(descriptions.into_transposed_iter())
        .map(|(name, description)| Item { name, description })
        .collect()
}

fn main() {
    assert_eq!(
        items(vec!["Alice", "Bob", "Charlie"], None),
        vec![
            Item {
                name: "Alice",
                description: None,
            },
            Item {
                name: "Bob",
                description: None,
            },
            Item {
                name: "Charlie",
                description: None,
            },
        ]
    );

    assert_eq!(
        items(
            vec!["Alice", "Bob", "Charlie"],
            Some(vec!["in Wonderland", "the builder", "likes chocolate"])
        ),
        vec![
            Item {
                name: "Alice",
                description: Some("in Wonderland"),
            },
            Item {
                name: "Bob",
                description: Some("the builder"),
            },
            Item {
                name: "Charlie",
                description: Some("likes chocolate"),
            },
        ]
    );
}
```

You can also use this function to iterate over all existing elements.
These two are equivalent:

```
use iter_transpose::IntoTransposedIter;
assert_eq!(
    Some((0..5))
        .into_transposed_iter()
        .unwrap_while_some()
        .collect::<Vec<_>>(),
    Some((0..5))
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
);
```

[`Option::transpose`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose
[`Result`]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
