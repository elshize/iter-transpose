use iter_transpose::IterTranspose;
use std::io::{BufRead, BufReader, Cursor};

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    id: String,
    value: u64,
}

impl Entry {
    pub fn new(id: String, value: Option<u64>) -> Self {
        Self {
            id,
            value: value.unwrap_or_else(Self::default_value),
        }
    }
    fn default_value() -> u64 {
        0
    }
}

fn load_entries<R: BufRead>(ids: R, values: Option<R>) -> Vec<Entry> {
    let mut values = values.map(|r| {
        r.lines().map(|line| {
            line.expect("cannot read line")
                .parse::<u64>()
                .expect("cannot parse as u64")
        })
    });
    ids.lines()
        .map(|line| {
            let id = line.expect("cannot read line");
            Entry::new(id, values.as_mut().and_then(|values| values.next()))
        })
        .collect()
}

fn load_entries_with_transpose<R: BufRead>(ids: R, values: Option<R>) -> Vec<Entry> {
    let values = values.map(|r| {
        r.lines().map(|line| {
            line.expect("cannot read line")
                .parse::<u64>()
                .expect("cannot parse as u64")
        })
    });
    ids.lines()
        .zip(values.transpose_into_iter())
        .map(|(line, value)| Entry::new(line.expect("cannot read line"), value))
        .collect()
}

trait IterT {
    type Iterable: IntoIterator;
    type TransposedItem;
    type Iter: Iterator<Item = Self::TransposedItem>;
}

impl<T: IntoIterator> IterT for Option<T> {
    type Iterable = T;
    type TransposedItem = Option<T::Item>;
    type Iter = iter_transpose::OptionTransposedIter<<Self::Iterable as IntoIterator>::IntoIter>;
}

fn main() {
    let ids = "A
B
C
D";
    let values = "1
2
3
4";
    assert_eq!(
        load_entries(BufReader::new(Cursor::new(ids)), None),
        vec![
            Entry {
                id: "A".into(),
                value: 0
            },
            Entry {
                id: "B".into(),
                value: 0
            },
            Entry {
                id: "C".into(),
                value: 0
            },
            Entry {
                id: "D".into(),
                value: 0
            },
        ]
    );
    assert_eq!(
        load_entries(
            BufReader::new(Cursor::new(ids)),
            Some(BufReader::new(Cursor::new(values)))
        ),
        vec![
            Entry {
                id: "A".into(),
                value: 1
            },
            Entry {
                id: "B".into(),
                value: 2
            },
            Entry {
                id: "C".into(),
                value: 3
            },
            Entry {
                id: "D".into(),
                value: 4
            },
        ]
    );
    assert_eq!(
        load_entries_with_transpose(BufReader::new(Cursor::new(ids)), None),
        vec![
            Entry {
                id: "A".into(),
                value: 0
            },
            Entry {
                id: "B".into(),
                value: 0
            },
            Entry {
                id: "C".into(),
                value: 0
            },
            Entry {
                id: "D".into(),
                value: 0
            },
        ]
    );
    assert_eq!(
        load_entries_with_transpose(
            BufReader::new(Cursor::new(ids)),
            Some(BufReader::new(Cursor::new(values)))
        ),
        vec![
            Entry {
                id: "A".into(),
                value: 1
            },
            Entry {
                id: "B".into(),
                value: 2
            },
            Entry {
                id: "C".into(),
                value: 3
            },
            Entry {
                id: "D".into(),
                value: 4
            },
        ]
    );
}
