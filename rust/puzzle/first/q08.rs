fn main() {
    let cnt = move_robot(Loc { x: 0, y: 0 }, vec![]);
    println!("{}", cnt);
}
fn move_robot(loc: Loc, mut history: Vec<Loc>) -> usize {
    for i in history.iter() {
        if i == &loc {
            return 0;
        }
    }
    history.push(loc);
    if history.len() == 13 {
        return 1;
    }
    let mut cnt: usize = 0;
    cnt += move_robot(loc.right(), history.clone());
    cnt += move_robot(loc.left(), history.clone());
    cnt += move_robot(loc.down(), history.clone());
    cnt += move_robot(loc.up(), history.clone());
    cnt
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Loc {
    x: i8,
    y: i8,
}
impl Loc {
    fn right(&self) -> Loc {
        Loc {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn left(&self) -> Loc {
        Loc {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn down(&self) -> Loc {
        Loc {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn up(&self) -> Loc {
        Loc {
            x: self.x,
            y: self.y - 1,
        }
    }
}
