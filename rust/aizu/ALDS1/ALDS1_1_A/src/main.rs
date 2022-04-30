fn main() {
    let loops: usize = get_input().parse().unwrap();
    let inputs: Vec<i32> = get_input().split(' ').map(|x| x.parse().unwrap()).collect();

    let mut result: Vec<i32> = vec![];
    for input in inputs.iter() {
        into_result(&mut result, *input);
        output(&result, &inputs, loops)
    }
}

fn into_result(result: &mut Vec<i32>, num: i32) {
    for (i, val) in result.iter().enumerate() {
        if val > &num {
            result.insert(i, num);
            return;
        }
    }
    result.push(num);
}

fn output(result: &[i32], input: &[i32], length: usize) {
    let result_len = result.len();

    output_merge(&result[0..result_len], &input[result_len..length]);
}

fn output_merge(result: &[i32], input: &[i32]) {
    let mut output = vec![];
    output.extend(result);
    output.extend(input);
    print_output(&output);
}

fn print_output(data: &[i32]) {
    println!(
        "{}",
        data.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_owned()
}

#[test]
fn test_output() {
    let data: Vec<i32> = vec![2, 5, 6];
    let data2: Vec<i32> = vec![3, 4, 9];

    output(&data.as_slice()[0..3], &data2.as_slice()[0..2]);
}
