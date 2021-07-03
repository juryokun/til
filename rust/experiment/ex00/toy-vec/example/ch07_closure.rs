fn apply_fn<F>(f: &F, ch: char)
where
    F: Fn(char) -> bool,
{
    assert!(f(ch));
}

fn apply_fn_mut<F>(f: &mut F, ch: char)
where
    F: FnMut(char) -> bool,
{
    assert!(f(ch));
}

fn apply_fn_once<F>(f: F, ch: char)
where
    F: FnOnce(char) -> bool,
{
    assert!(f(ch));
}

fn main() {
    let s1 = "read-only";
    let mut lookup = |ch| s1.find(ch).is_some();

    apply_fn(&lookup, 'r');
    apply_fn_mut(&mut lookup, 'o');
    apply_fn_once(lookup, 'y');
}
