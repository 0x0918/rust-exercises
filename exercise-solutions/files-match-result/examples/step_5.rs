use std::fs::File;
use std::io::{BufRead, BufReader};
use url::Url;

fn parse_line(line: String) -> Option<Url> {
    match Url::parse(&line) {
        Ok(u) => Some(u),
        Err(_e) => None,
    }
}

fn main() {
    let open_result = File::open("src/data/content.txt");

    let file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let line = match line {
            Ok(content) => content,

            Err(e) => panic!("Error reading line {}", e),
        };

        let url = parse_line(line);

        match url {
            Some(line) => println!("{}", line),
            None => continue,
        }
    }
}

#[test]
fn correct_url() {
    assert!(parse_line(String::from("https://example.com")).is_some())
}

#[test]
fn no_url() {
    assert!(parse_line(String::from("abcdf")).is_none())
}
