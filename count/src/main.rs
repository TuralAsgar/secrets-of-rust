use std::io::{stdin, BufRead, Error, ErrorKind, Read};

fn main() {
    let res = count_lines(stdin().lock());
    match res {
        Ok(lines) => println!("{lines} lines"),
        Err(e) => println!("{e}"),
    }
}

pub fn count_lines(input: impl BufRead) -> Result<usize, Error> {
    let mut count = 0;
    for line in input.lines() {
        line?;
        count += 1
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use super::*;

    #[test]
    fn count_lines_fn_counts_lines_in_input() {
        let input = Cursor::new("line 1\nline 2\n");
        let lines = count_lines(input).unwrap();
        assert_eq!(lines, 2, "wrong line count");
    }

    #[test]
    fn count_lines_fn_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count_lines(reader);
        assert!(result.is_err(), "no error returned");
    }
}

struct ErrorReader;

impl Read for ErrorReader {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize, Error> {
        Err(Error::new(ErrorKind::Other, "oh no"))
    }
}
