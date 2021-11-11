use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::from_utf8;

fn load_json<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    for result in BufReader::new(File::open(path)?).lines() {
        let l = result?;
        println!("{}", l);

        let c = l.as_bytes();
        let mut pos = 0;
        while pos < c.len() {
            let cc = [c[pos]];
            let n = from_utf8(&cc).unwrap();
            println!("{}", n);
            pos = pos + 1;
        }
    }
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
