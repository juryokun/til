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
                b'{' => {
                    let moji = fetch_charactor(c, pos, pos + 1);
                    words.push(moji.charactor);
                    pos = moji.end + 1;
                }
                b'}' => {
                    let moji = fetch_charactor(c, pos, pos + 1);
                    words.push(moji.charactor);
                    pos = moji.end + 1;
                }
                b',' => {
                    let moji = fetch_charactor(c, pos, pos + 1);
                    words.push(moji.charactor);
                    pos = moji.end + 1;
                }
                b'"' => {
                    let moji = identify(c, pos);
                    words.push(moji.charactor);
                    pos = moji.end + 1;
                }
                _ => pos = pos + 1,
            }
            // let n = from_utf8(&cc).unwrap();
            // match n {
            //     "{" => words.push(n.to_string()),
            // }
            // println!("{}", n);
        }
    }
    println!("{:?}", words);
    Ok(())
}

struct MOJI {
    charactor: String,
    start: usize,
    end: usize,
}

fn identify(line: &[u8], mut pos: usize) -> MOJI {
    let start_pos = pos;
    pos = pos + 1;
    let mut end_pos = 0;
    while pos < line.len() {
        if line[pos] == b'"' {
            end_pos = pos + 1;
            break;
        }
        pos = pos + 1;
    }
    fetch_charactor(line, start_pos, end_pos)
}

fn fetch_charactor(line: &[u8], start: usize, end: usize) -> MOJI {
    let moji = from_utf8(&line[start..end]).unwrap().to_string();
    MOJI {
        charactor: moji,
        start: start,
        end: end,
    }
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
