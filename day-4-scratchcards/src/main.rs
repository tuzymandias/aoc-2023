use std::collections::VecDeque;

const INPUT: &'static str = include_str!("input.txt");

fn parse_line(line: &str) -> u32 {
    // Splitting by Delimiters 
    let (_, numbers) = line.split_once(": ").unwrap_or_default();
    let (winners_str, randoms_str) = numbers.split_once(" | ").unwrap_or_default();

    // Split winning numbers by whitespace and collect it into a vector of numbers
    // With map_windows we can use constant sized array here instead of vec!
    let winners = winners_str.split_whitespace()
        .map(str::parse::<u32>)
        .map(Result::unwrap_or_default)
        .collect::<Vec<u32>>();

    // Likewise split the random numbers and parse into a number
    let randoms = randoms_str.split_whitespace()
        .map(str::parse::<u32>)
        .map(Result::unwrap_or_default);

    // Calculate number of matching numbers in winners and randoms (i.e. set intersection)
    let num_matches = randoms.fold(0, |matching_count, random_num| matching_count + winners.contains(&random_num) as u32);
    num_matches
}

fn main() {
    let score = INPUT.lines()
        .map(|line| parse_line(line))
        .map(|num_matches| if num_matches == 0 { 0 } else { 2_u32.pow(num_matches-1) })
        .sum::<u32>();

    println!("Total Score: {}", score);

    // Initialize all card to have 1 count, introduce the state via `scan`
    let card_counts = std::iter::repeat(1).take(INPUT.lines().count()).collect::<VecDeque<u32>>();
    let num_cards = INPUT.lines()
        .map(|line| parse_line(line))
        .scan(card_counts, |card_counts, num_matches| {
            // Take count of current card
            let multiplier = card_counts.pop_front();

            // Add `num_matches` to the first `multiplier` elements in `card_counts`
            let adder = std::iter::repeat(multiplier.unwrap_or(0)).take(num_matches as usize);
            card_counts.iter_mut().by_ref().zip(adder).for_each(|(count, adder)| *count += adder);

            multiplier
        })
        .sum::<u32>();

    println!("Total Card Count: {}", num_cards);
}
