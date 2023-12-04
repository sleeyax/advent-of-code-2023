advent_of_code::solution!(4);

fn parse_cards(input: &str) -> Vec<u32> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .map(|line| {
            let line = line.split(":").last().unwrap().trim();

            let split = line.split("|").collect::<Vec<&str>>();
            let winning_cards = parse_cards(split[0]);
            let have_cards = parse_cards(split[1]);

            let good_cards = have_cards
                .iter()
                .filter_map(|c| winning_cards.contains(c).then_some(*c))
                .collect::<Vec<_>>();
            let mut points = 0;
            if good_cards.len() != 0 {
                points = 2_u32.pow((good_cards.len() as u32) - 1);
            }

            points
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
