#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Location（位置情報）
/// Loc(4,6)であれば、５文字目から７文字目までの区間を表す
struct Loc(usize, usize);

impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

/// アノテーション。あたいにさまざまなデータを持たせたもの。ここではLocを持たせる
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

fn main() {
    println!("Hello, world!");
}
