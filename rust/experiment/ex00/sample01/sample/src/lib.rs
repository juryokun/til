
pub fn abs(x: i32) -> u32 {
    if x > 0 {
        return x as u32;
    } else {
        return -x as u32;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs(2), 2);
    }

}
