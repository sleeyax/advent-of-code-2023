advent_of_code::solution!(1);

const MSB: u8 = 0xF0; // 1111 0000
const LSB: u8 = 0x0F; // 0000 1111
const NUMBER: u8 = 0x30;  // 0011 0000

fn read_code(line: &str) -> u32 {
    let mut numbers = line.as_bytes().iter().filter(|c| *c & MSB == NUMBER);
    let start = numbers.next().unwrap() & LSB;
    let end = numbers.last().unwrap_or(&start) & LSB;
    (start * 10 + end) as u32 // multiplication by 10 is safe as long as each number is in range 0..9
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(read_code).sum::<u32>().into()
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
