This crate provides `IterTranspose` trait that provides methods
to transforms `Option<I: IntoIterator>` into `impl Iterator<Option<I::Item>>`
and `Result<I: IntoIterator, E>` into `impl Iterator<Result<I:Item, E>>`.

# Documentation

For both overview and detailed information, check out the
[Documentation](https://elshize.github.io/iter-transpose/).
