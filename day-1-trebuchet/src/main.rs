const INPUT: &'static str = include_str!("input.txt");
const RADIX: u32 = 10;

fn compute_calibration_value(line: &str) -> i32 {
    let (first, last) = line.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(RADIX).unwrap() as i32)
        .fold((None, None), |(first, last), number | {
            match (first, last) {
                (None, None) => (Some(number), Some(number)),
                (Some(a), _) => (Some(a), Some(number)),
                _ => unreachable!()
            }
        });

    let (first, last) = (first.unwrap_or(0), last.unwrap_or(0));

    first * 10 + last
}

fn main() {
    let sum_cv = INPUT.lines()
        .map(|line| compute_calibration_value(line))
        .fold(0, |sum, cv| sum + cv);

    println!("{}", sum_cv)
}
