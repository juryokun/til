extern crate regex;
use eval::{eval, to_value};
use regex::Regex;
use std::collections::HashSet;

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
    let vecs = make_permutation(uniq.len());

    // アルファベットと数値の対応関係をtargetに反映
    let mut cnt = 0;
    for v in vecs {
        let mut tmp = target.to_string();
        let mut rep_cnt = 0;
        for (i, c) in uniq.iter().enumerate() {
            let num = &v[i].to_string();
            if num == &'0'.to_string() {
                if is_exist(heads.clone(), c) {
                    break;
                }
            }
            tmp = tmp.replace(&c.to_string(), num);
            rep_cnt += 1;
            if rep_cnt == uniq.len() {
                // evalで計算しtrueになったらカウント
                if eval(&tmp).unwrap() == true {
                    cnt += 1;
                }
            }
        }
    }

    // 値を出力
    println!("{}", cnt);
}
fn is_exist(heads: Vec<char>, c: &char) -> bool {
    for i in heads.iter() {
        if i == c {
            return true;
        }
    }
    false
}
fn make_permutation(n: usize) -> Vec<Vec<usize>> {
    let mut vecs: Vec<Vec<usize>> = vec![Vec::new(); factorial(n)];
    let nums: Vec<usize> = (0..n).collect();
    let indexes: Vec<usize> = (0..factorial(n)).collect();
    push_recusive(nums, indexes, &mut vecs);
    vecs
}

fn push_recusive<T: Clone>(
    nums: Vec<T>,
    indexes: Vec<usize>,
    vecs: &mut Vec<Vec<T>>,
) -> &mut Vec<Vec<T>> {
    if nums.len() == 0 {
        return vecs;
    }
    let block_size = factorial(nums.len() - 1);
    for (block_index, num) in nums.iter().enumerate() {
        for inner_index in 0..block_size {
            let index = indexes[block_size * block_index + inner_index];
            vecs[index].push(num.clone());
        }
        let new_nums = {
            let mut tmp = nums.clone();
            tmp.remove(block_index);
            tmp
        };
        let new_indexes: Vec<usize> = {
            let slice = &indexes[(block_size * block_index)..(block_size * (block_index + 1))];
            slice.to_vec()
        };
        push_recusive(new_nums, new_indexes, vecs);
    }
    vecs
}

fn factorial(i: usize) -> usize {
    if i <= 1 {
        1
    } else {
        (2..=i).fold(1, |acc, x| acc * x)
    }
}
