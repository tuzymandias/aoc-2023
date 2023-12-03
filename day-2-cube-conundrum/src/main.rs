use std::cmp;
use std::ops::Add;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Default, Debug)]
struct RGB(u32, u32, u32);

impl Add for RGB {
    fn add(self, other: RGB) -> RGB {
        let RGB(a_r, a_g, a_b) = self;
        let RGB(b_r, b_g, b_b) = other;
        RGB(a_r + b_r, a_g + b_g, a_b + b_b)
    }

    type Output = RGB;
}

fn possible(RGB(r, g, b): &RGB) -> bool {
    const RED_LIMIT: u32 = 12;
    const GREEN_LIMIT: u32 = 13;
    const BLUE_LIMIT: u32 = 14;

    return *r <= RED_LIMIT && *g <= GREEN_LIMIT && *b <= BLUE_LIMIT
}

fn max_of_each(RGB(a_r, a_g, a_b): RGB, RGB(b_r, b_g, b_b): RGB) -> RGB {
    RGB(cmp::max(a_r, b_r), cmp::max(a_g, b_g), cmp::max(a_b, b_b))
}

fn split_game(game: &str) -> (&str, &str) {
    let mut iter = game.split(": ");
    return (iter.next().unwrap(), iter.next().unwrap())
}

fn parse_set(set: &str) -> RGB {
    set.split(", ")
        .map(|cube_count_str| {
            let mut iter = cube_count_str.split(' ');
            let (r, g, b) = match (iter.next().map(|num_str| num_str.parse::<u32>()).unwrap(), iter.next()) {
                (Ok(count), Some("red")) => (count, 0, 0),
                (Ok(count), Some("green")) => (0, count, 0),
                (Ok(count), Some("blue")) => (0, 0, count),
                _ => (0, 0, 0),
            };
            RGB(r, g, b)})
        .fold(RGB::default(), RGB::add)
}

fn parse_sets(sets: &str) -> RGB {
    sets.split("; ").map(|set| parse_set(set)).fold(RGB::default(), max_of_each)
}

fn parse_game(game: &str) -> u32 {
    let mut iter = game.split(" ");
    iter.next();
    iter.next().unwrap().parse::<u32>().unwrap()
}

fn main() {
    let sum_game_num = INPUT.lines()
        .map(|line| split_game(line))
        .map(|(game, sets)| (parse_game(game), parse_sets(sets)))
        .filter(|(_, bag)| possible(bag))
        .fold(0, |sum, (game_num, _)| sum + game_num);

    println!("Sum Game Num {:?}", sum_game_num);

    let sum_power = INPUT.lines()
        .map(|line: &str| split_game(line))
        .map(|(_, sets)| parse_sets(sets))
        .fold(0, |sum, RGB(r, g, b)| sum + r * g * b);

    println!("Sum Game Power {:?}", sum_power);
}
