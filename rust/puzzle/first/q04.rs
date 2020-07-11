fn main() {
    println!("{}", cutbar(20, 3, 1));
    println!("{}", cutbar(100, 5, 1))
}

fn cutbar(size: usize, pnum: usize, cur: usize) -> usize {
    if cur >= size {
        return 0;
    } else if cur <= pnum {
        return 1 + cutbar(size, pnum, cur * 2);
    }
    return 1 + cutbar(size, pnum, cur + pnum);
}

#[test]
fn test_cutbar() {
    let data = vec![[20, 3, 8], [100, 5, 22]];

    for target_data in data.iter() {
        assert_eq!(cutbar(target_data[0], target_data[1], 1), target_data[2]);
    }
}
