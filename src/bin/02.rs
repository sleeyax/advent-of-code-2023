advent_of_code::solution!(2);

struct Game {
    pub id: usize,
    colorsets: Vec<Vec<(usize, Color)>>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let split = value.split(": ").collect::<Vec<&str>>();
        let id = split[0].replace("Game ", "").parse::<usize>().unwrap();
        let value = split[1];

        let colorset_strings = value.split("; ");
        let mut colorsets = Vec::new();
        for colorset_string in colorset_strings {
            let colors = colorset_string.split(", ");
            let mut colorset = Vec::new();
            for color in colors {
                let split = color.split(" ").collect::<Vec<&str>>();
                let amount = split[0].parse::<usize>().unwrap();
                let color = Color::from(split[1]);
                colorset.push((amount, color));
            }
            colorsets.push(colorset);
        }

        Game {id, colorsets}
    }
}

impl Game {
    /// Returns true if the game is possible, false otherwise.
    pub fn play(&self, bag: Vec<(usize, Color)>) -> bool {
        for (amount, color) in bag {
            for colorset in &self.colorsets {
                if colorset.iter().find(|(a, c)| c == &color && a > &amount).is_some() {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Unknown color {}", s),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut result = 0;

    for line in input.lines() {
        let game = Game::from(line);
        let bag = vec![(12, Color::Red), (13, Color::Green), (14, Color::Blue)];
        if game.play(bag) {
             result += game.id;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
