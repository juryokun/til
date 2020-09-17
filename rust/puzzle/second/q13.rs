extern crate regex;
use eval::eval;
use regex::Regex;
use std::collections::HashSet;

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let target = "READ+WRITE+TALK==SKILL";
    // 正規表現でアルファベットだけ抜き出す
    let re = Regex::new(r"(\+|=|-|/)").unwrap();
    let result = re.replace_all(target, "");

    // アルファベットの重複削除
    let mut uniq = HashSet::new();
    for val in result.chars() {
        uniq.insert(val);
    }
    let heads = re
        .split(target)
        .map(|w| w.chars().next().unwrap_or('0'))
        .collect::<Vec<_>>();
    let should_not_zero_loc = set_heads_loc(&heads, &uniq);

    // 各アルファベットに数値割当（頭0省く）
    let mut vecs = permutations(uniq.len() as i32, uniq.len() as i32);
    let vecs_a = vecs.split_off(vecs.len() / 2);

    // アルファベットと数値の対応関係をtargetに反映
    let counter = Arc::new(Mutex::new(0));
    let uniq_a = Arc::new(uniq);
    let should_not_zero_loc_a = Arc::new(should_not_zero_loc);

    // 並列処理1
    let handle1 = spawn_count_equal(
        vecs,
        &counter,
        &uniq_a,
        &should_not_zero_loc_a,
        target.to_string(),
    );

    // 並列処理2
    let handle2 = spawn_count_equal(
        vecs_a,
        &counter,
        &uniq_a,
        &should_not_zero_loc_a,
        target.to_string(),
    );

    // 並列処理の終了を待つ
    handle1.join().unwrap();
    handle2.join().unwrap();

    // 値を出力
    println!("{}", *counter.lock().unwrap());
}

fn spawn_count_equal(
    vec: Vec<Vec<i32>>,
    counter: &Arc<Mutex<i32>>,
    uniq: &Arc<HashSet<char>>,
    should_not_zero_loc: &Arc<Vec<i32>>,
    target: String,
) -> thread::JoinHandle<()> {
    let cnt = Arc::clone(&counter);
    let uniq = Arc::clone(&uniq);
    let should_not_zero_loc = Arc::clone(&should_not_zero_loc);
    thread::spawn(move || {
        count_equal(vec, target, uniq, should_not_zero_loc, cnt);
    })
}

fn count_equal(
    vecs: Vec<Vec<i32>>,
    target: String,
    uniq: Arc<HashSet<char>>,
    should_not_zero_loc: Arc<Vec<i32>>,
    cnt: Arc<Mutex<i32>>,
) {
    for v in vecs {
        if contain_head_zero(&should_not_zero_loc, &v) {
            continue;
        }

        let mut tmp = target.clone();
        for (c, n) in uniq.iter().zip(v.iter()) {
            let num = &n.to_string();
            tmp = tmp.replace(&c.to_string(), num);
        }
        if eval(&tmp).unwrap() == true {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        }
    }
}
fn contain_head_zero(should_not_zero_loc: &Vec<i32>, v: &Vec<i32>) -> bool {
    let mut is_head_zero = false;
    for loc in should_not_zero_loc.iter() {
        if (0 as i32) == v[(*loc as usize)] {
            is_head_zero = true;
            break;
        }
    }
    is_head_zero
}
fn set_heads_loc(heads: &Vec<char>, uniq: &HashSet<char>) -> Vec<i32> {
    let mut should_not_zero_loc: Vec<i32> = vec![];
    for (i, c) in uniq.iter().enumerate() {
        // if heads.iter().any(|i| i == c) {
        if heads.contains(c) {
            should_not_zero_loc.push(i as i32);
        }
    }
    should_not_zero_loc
}

fn permutations(n: i32, m: i32) -> Vec<Vec<i32>> {
    fn perm(n: i32, m: i32, xs: &mut Vec<i32>, v: &mut Vec<Vec<i32>>) {
        if m == 0 {
            v.push(xs.clone());
        } else {
            for x in 0..n {
                if !xs.contains(&x) {
                    xs.push(x);
                    perm(n, m - 1, xs, v);
                    xs.pop();
                }
            }
        }
    }
    let mut v = vec![];
    perm(n, m, &mut vec![], &mut v);
    v
}

#[test]
fn test_perm() {
    let v = permutations(3, 3);
    let assert_v = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    assert_eq!(v, assert_v);
}
