use anyhow::Result;
use std::env::args;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    let args: Vec<_> = args().skip(1).collect();
    if args.is_empty() {
        if fs::exists("logbook.txt")? {
            let text = fs::read_to_string("logbook.txt")?;
            print!("{text}");
        } else {
            println!("Logbook is empty");
        }
    } else {
        let mut logbook = File::options()
            .create(true)
            .append(true)
            .open("logbook.txt")?;
        writeln!(logbook, "{}", args.join(" "))?;
    }
    Ok(())
}

pub fn read(path: &str) -> Result<Option<String>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_returns_none_if_file_does_not_exist() {
        let text = read("tests/data/bogus.txt").unwrap();
        assert_eq!(text, None, "expected to return none");
    }

    #[test]
    fn read_returns_none_for_empty_file() {
        let text = read("tests/data/empty.txt").unwrap();
        assert_eq!(text, None, "expected to return none");
    }
}
