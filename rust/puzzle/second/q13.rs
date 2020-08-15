use eval::{eval, to_value};

fn main() {
    let target = "read+write+talk=skill";
    let vec: Vec<&str> = target.split("=").collect();
    let left = vec[0];
    let right = vec[1];

    // leftから文字列の重複削除、計算記号の削除
    // let chars =

    // 0-9を各文字に割当(置換)

    // evalで計算
}
