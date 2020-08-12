fn main() {
    let european: Vec<usize> = vec![
        0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27, 13, 36, 11, 30, 8, 23, 10, 5, 24, 16, 33, 1,
        20, 14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26,
    ];
    let american: Vec<usize> = vec![
        0, 28, 9, 26, 30, 11, 7, 20, 32, 17, 5, 22, 34, 15, 3, 24, 36, 13, 1, 0, 27, 10, 25, 29,
        12, 8, 19, 31, 18, 6, 21, 33, 16, 4, 23, 35, 14, 2,
    ];
    let mut cnt = 0;
    for i in 2..=36 {
        if max_sum(european.clone(), i) < max_sum(american.clone(), i) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}

fn max_sum(table: Vec<usize>, size: usize) -> usize {
    let mut sum: usize = table[0..size].iter().sum();
    let mut tmp: usize = sum.clone();
    for i in 0..table.len() {
        tmp += table[(size + i) % table.len()];
        tmp -= table[i];
        if tmp > sum {
            sum = tmp;
        }
    }
    sum
}

#[test]
fn test_max_sum() {
    let data = vec![5, 1, 0, 4, 3];
    assert_eq!(max_sum(data, 2), 8);

    let data = vec![6, 1, 4, 4, 0];
    assert_eq!(max_sum(data, 2), 8);
}
