#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub fn hash(x: u64) -> u64 {
    let mut y = x;
    for _ in 0..512 {
        y = y<<5;
        y = y ^ x;
    }
    y
}
