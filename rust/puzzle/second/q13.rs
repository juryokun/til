extern crate regex;
use eval::{eval, to_value};
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let target = "READ+WRITE+TALK=SKILL";
    // 正規表現でアルファベットだけ抜き出す
    let re = Regex::new(r"[^a-zA-Z]").unwrap();
    let words = re.split(target).collect::<Vec<_>>();

    // 頭文字を抜き出す
    let heads = words
        .iter()
        .map(|w| w.chars().next().unwrap())
        .collect::<Vec<_>>();

    // アルファベットの重複削除
    let mut uniq = HashSet::new();
    for val in words.join("").chars() {
        uniq.insert(val);
    }
    println!("{:?}", uniq);

    // 各アルファベットに数値割当（頭0省く）

    // アルファベットと数値の対応関係をtargetに反映

    // evalで計算しtrueになったらカウント
}
