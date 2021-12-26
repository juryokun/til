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

        macro_rules! push_charactor {
            ($moji: expr) => {
                words.push($moji.charactor);
                pos = $moji.end;
            };
        }
        while pos < c.len() {
            let cc = c[pos];
            match cc {
                b'{' => {
                    push_charactor!(fetch_charactor(c, pos, pos + 1));
                }
                b'}' => {
                    push_charactor!(fetch_charactor(c, pos, pos + 1));
                }
                b',' => {
                    push_charactor!(fetch_charactor(c, pos, pos + 1));
                }
                b':' => {
                    push_charactor!(fetch_charactor(c, pos, pos + 1));
                }
                b'[' => {
                    push_charactor!(fetch_charactor(c, pos, pos + 1));
                }
                b']' => {
                    push_charactor!(fetch_charactor(c, pos, pos + 1));
                }
                b'"' => {
                    push_charactor!(identify(c, pos));
                }
                _ => pos = pos + 1,
            }
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
            if line[pos - 1] == b'\\' {
                pos = pos + 1;
                continue;
            }
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

type Jsons = Vec<Json>;
#[derive(Debug)]
enum Json {
    Recursive { key: String, value: Box<Json> },
    End { key: String, value: String },
}

fn test_data() -> Jsons {
    let json1 = Json::End {
        key: "aaa".to_string(),
        value: "aval".to_string(),
    };
    let json2 = Json::Recursive {
        key: "bbb".to_string(),
        value: Box::new(json1),
    };
    let json3 = Json::Recursive {
        key: "ccc".to_string(),
        value: Box::new(json2),
    };

    vec![json3]
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
    #[test]
    fn print_json() {
        println!("{:?}", test_data());
    }
}
