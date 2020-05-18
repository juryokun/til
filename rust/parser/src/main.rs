#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    Loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, Loc: Loc) -> Self {
        Self { value, Loc }
    }
}

fn main() {
    println!("Hello, world!");
}
