extern crate regex;
use eval::{eval, to_value};
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let target = "READ+WRITE+TALK=SKILL";
    // 正規表現でアルファベットだけ抜き出す
    let re = Regex::new(r"(\+|=|-)").unwrap();
    let result = re.replace_all(target, "");
    let mut uniq = HashSet::new();
    for val in result.chars() {
        uniq.insert(val);
    }
    println!("{:?}", uniq);

    // アルファベットの重複削除

    // 各アルファベットに数値割当（頭0省く）

    // アルファベットと数値の対応関係をtargetに反映

    // evalで計算しtrueになったらカウント
}
