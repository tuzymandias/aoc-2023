use std::cmp;

const INPUT: &'static str = include_str!("input.txt");

// Modified from: https://users.rust-lang.org/t/iterator-over-windows-of-chars/17841/2
// Iterate through substrings of a string with window size of n
// Window gets smaller as it approaches the end
fn str_windows(line: &str, n: usize) -> impl Iterator<Item = &str> {
    line.char_indices()
        .map(move |(i, _)| &line[i..cmp::min(i+n, line.len())])
}

fn parse(s: &str) -> Option<i32> {
    match s {
        _ if s.starts_with("one") | s.starts_with('1') => Some(1),
        _ if s.starts_with("two") | s.starts_with('2') => Some(2),
        _ if s.starts_with("three") | s.starts_with('3') => Some(3),
        _ if s.starts_with("four") | s.starts_with('4') => Some(4),
        _ if s.starts_with("five") | s.starts_with('5') => Some(5),
        _ if s.starts_with("six") | s.starts_with('6') => Some(6),
        _ if s.starts_with("seven") | s.starts_with('7') => Some(7),
        _ if s.starts_with("eight") | s.starts_with('8') => Some(8),
        _ if s.starts_with("nine") | s.starts_with('9') => Some(9),
        _ => None
    }
}

fn compute_calibration_value(line: &str) -> i32 {
    let pair = str_windows(line, 5)
        .filter_map(|window| parse(window))
        .fold((None, None), |(first, last), number| {
            match (first, last) {
                (None, None) => (Some(number), Some(number)),
                (Some(a), _) => (Some(a), Some(number)),
                _ => unreachable!()
            }
        });

    match pair {
        (Some(first), Some(last)) => first * 10 + last,
        _ => 0
    }
}

fn main() {
    let sum_cv = INPUT.lines()
        .map(|line| compute_calibration_value(line))
        .fold(0, |sum, cv| sum + cv);

    println!("{}", sum_cv);
}
