use std::fs::File;
use std::io::{ self, BufRead, BufReader };

/// Very naive implementation of FizzBuzz
pub fn fizz_buzz(i: u32) -> String {
    if i % 3 == 0 {
        if i % 5 == 0 {
            "FizzBuzz".to_owned()
        } else {
            "Fizz".to_owned()
        }
    } else if i % 5 == 0 {
        "Buzz".to_owned()
    } else {
        format!("{i}")
    }
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}

// TODO Write a unit test, using the contents of `fizzbuzz.out` file
// to compare.
// You can use the `include_str!()` macro to include file
// contents as `&str` in your artifact.

#[cfg(test)]
mod tests {
    use crate::{read_lines, fizz_buzz};

    #[test]
    fn fizz_buzz_test() {
        let lines = read_lines("fizzbuzz.out".to_string());
        let mut i : u32 = 1;
        for line in lines {
            assert_eq!(fizz_buzz(i), line.unwrap());
            i += 1;
        }
    }
}