use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::from_utf8;

fn load_json<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let mut words: Vec<String> = vec![];
    for result in BufReader::new(File::open(path)?).lines() {
        let l = result?;
        // println!("{}", l);

        let c = l.as_bytes();
        let mut pos = 0;
        while pos < c.len() {
            let cc = c[pos];
            match cc {
                b'{' => words.push(from_utf8(&[cc]).unwrap().to_string()),
                b'}' => words.push(from_utf8(&[cc]).unwrap().to_string()),
                b',' => words.push(from_utf8(&[cc]).unwrap().to_string()),
                b'"' => {
                    pos = pos + 1;
                    let start_pos = pos;
                    let mut end_pos = 0;
                    while pos < c.len() {
                        if c[pos] == b'"' {
                            end_pos = pos;
                            break;
                        }
                        pos = pos + 1;
                    }
                    if end_pos == 0 {
                        words.push("".to_string());
                    }
                    words.push(from_utf8(&c[start_pos..end_pos]).unwrap().to_string());
                }
                _ => (),
            }
            // let n = from_utf8(&cc).unwrap();
            // match n {
            //     "{" => words.push(n.to_string()),
            // }
            // println!("{}", n);
            pos = pos + 1;
        }
    }
    println!("{:?}", words);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn print_items() {
        let file = "data/sample.json";
        let file_path = Path::new(file);

        load_json(file_path);
    }
}
