fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

#[derive(PartialEq)]
enum Answer {
    Correct,
    Fale,
}

// fn ref_vector(vector: &Vec<Answer>) -> i32 {
fn ref_vector(vector: &[Answer]) -> i32 {
    let mut count = 0;
    for item in vector.iter() {
        if item == &Answer::Correct {
            count += 1;
        }
    }

    count
}

fn ref_box(boxi: &Answer) -> bool {
    // fn ref_box(boxi: &Box<Answer>) -> bool {
    // if boxi.as_ref() == &Answer::Correct {
    if boxi == &Answer::Correct {
        return true;
    }
    false
}

fn main() {
    str_run();
    vec_run();
    box_run();
}

fn str_run() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    // This works fine, but the following two lines would fail:
    println!("Ferris: {}", three_vowels("Ferris"));
    println!("Curious: {}", three_vowels("Curious"));
}

fn vec_run() {
    let mut answers: Vec<Answer> = vec![];
    answers.push(Answer::Correct);
    answers.push(Answer::Fale);

    println!("{}", ref_vector(&answers));

    let answers2: [Answer; 2] = [Answer::Correct, Answer::Fale];
    println!("{}", ref_vector(&answers2));
}

fn box_run() {
    let boxi: Box<Answer> = Box::new(Answer::Correct);
    println!("{}", ref_box(&boxi));

    let ans: Answer = Answer::Correct;
    println!("{}", ref_box(&ans));
}
