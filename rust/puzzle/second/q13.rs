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

    // 各アルファベットに数値割当（頭0省く）
    let mut vecs = permutations(uniq.len() as i32, uniq.len() as i32);

    let vecs_a = vecs.split_off(vecs.len() / 2);

    // アルファベットと数値の対応関係をtargetに反映
    let counter = Arc::new(Mutex::new(0));
    let uniq_a = Arc::new(uniq);
    let heads_a = Arc::new(heads);

    // 並列処理1
    let cnt1 = Arc::clone(&counter);
    let uniq1 = Arc::clone(&uniq_a);
    let heads1 = Arc::clone(&heads_a);
    let handle1 = thread::spawn(move || {
        count_equal(vecs, target.to_string(), uniq1, heads1, cnt1);
    });

    // 並列処理2
    let cnt2 = Arc::clone(&counter);
    let uniq2 = Arc::clone(&uniq_a);
    let heads2 = Arc::clone(&heads_a);
    let handle2 = thread::spawn(move || {
        count_equal(vecs_a, target.to_string(), uniq2, heads2, cnt2);
    });

    // 並列処理の終了を待つ
    handle1.join().unwrap();
    handle2.join().unwrap();
    // 値を出力
    println!("{}", *counter.lock().unwrap());
}
fn count_equal(
    vecs: Vec<Vec<i32>>,
    target: String,
    uniq: Arc<HashSet<char>>,
    heads: Arc<Vec<char>>,
    cnt: Arc<Mutex<i32>>,
) {
    for v in vecs {
        let mut tmp = target.to_string();
        let mut rep_cnt = 0;
        for (i, c) in uniq.iter().enumerate() {
            let num = &v[i].to_string();
            if num == &'0'.to_string() {
                if is_exist(&heads, c) {
                    break;
                }
            }
            tmp = tmp.replace(&c.to_string(), num);
            rep_cnt += 1;
            if rep_cnt == uniq.len() {
                // evalで計算しtrueになったらカウント
                if eval(&tmp).unwrap() == true {
                    let mut num = cnt.lock().unwrap();
                    *num += 1;
                }
            }
        }
    }
}

fn is_exist(heads: &Vec<char>, c: &char) -> bool {
    for i in heads.iter() {
        if i == c {
            return true;
        }
    }
    false
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

#[test]
fn test_is_exist() {
    let heads = vec!['H', 'I', 'B', 'P'];
    assert_eq!(is_exist(&heads, &'B'), true);

    assert_eq!(is_exist(&heads, &'A'), false);
}
