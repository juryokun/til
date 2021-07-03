use std::collections::HashMap;
fn main() {
    let mut v1 = vec![0, 1, 2, 3];
    v1.push(4);
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());
    v1.push(4);
    v1.push(4);
    v1.push(4);
    v1.push(4);
    v1.push(4);
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());

    let s1 = v1.into_boxed_slice();

    let v2 = s1.into_vec();
    println!("v1 len: {}, capacity: {}", v2.len(), v2.capacity());

    let m2 = vec![("a", 1), ("b", 3)]
        .into_iter()
        .collect::<HashMap<_, _>>();
    println!("{:?}", m2.get("b"));
    let mut m1 = HashMap::new();
    m1.insert("a", 1);
    m1.insert("b", 3);
    println!("{:?}", m1);
    println!("{:?}", m1.get("b"));
}
