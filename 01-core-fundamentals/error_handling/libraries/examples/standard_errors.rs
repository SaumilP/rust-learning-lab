use std::fs::File;
use std::io;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

fn parse_count(text: &str) -> Result<u32, ParseIntError> {
    text.trim().parse()
}

fn decode_text(bytes: Vec<u8>) -> Result<String, FromUtf8Error> {
    String::from_utf8(bytes)
}

fn open_required_file(path: &str) -> Result<File, io::Error> {
    File::open(path)
}

fn main() {
    match parse_count("forty-two") {
        Ok(count) => println!("Count: {count}"),
        Err(error) => println!("ParseIntError: {error}"),
    }

    match decode_text(vec![0xf0, 0x28, 0x8c, 0x28]) {
        Ok(text) => println!("Text: {text}"),
        Err(error) => println!("FromUtf8Error: {error}"),
    }

    match open_required_file("missing-learning-example.txt") {
        Ok(_) => println!("Opened the file"),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            println!("io::Error: file was not found")
        }
        Err(error) => println!("io::Error: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_count() {
        assert_eq!(parse_count(" 42 "), Ok(42));
    }

    #[test]
    fn reports_invalid_utf8() {
        assert!(decode_text(vec![0xff]).is_err());
    }

    #[test]
    fn distinguishes_not_found_io_errors() {
        let error = open_required_file("missing-learning-test-file.txt").unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }
}
