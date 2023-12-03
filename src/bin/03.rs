advent_of_code::solution!(3);

fn is_symbol(c: char) -> bool {
    let b = c as u8;
    let is_digit = b >= b'0' && b <= b'9';
    !is_digit && c != '.'
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut total = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for (i, row) in grid.iter().enumerate() {
        let mut number: usize = 0;
        let mut symbol_found = false;

        for (j, col) in row.iter().enumerate() {
            if col.is_digit(10) {
                number = (number * 10) + col.to_digit(10).unwrap() as usize;

                if !symbol_found {
                    // look left, right, up, down and diagonally
                    // I feel like this is a naive approach but it works  ¯\_(ツ)_/¯
                    if (j > 0 && is_symbol(grid[i][j - 1]))
                        || (j < row.len() - 1 && is_symbol(grid[i][j + 1]))
                        || (i > 0 && is_symbol(grid[i - 1][j]))
                        || (i < grid.len() - 1 && is_symbol(grid[i + 1][j]))
                        || (i > 0 && j > 0 && is_symbol(grid[i - 1][j - 1]))
                        || (i > 0 && j < row.len() - 1 && is_symbol(grid[i - 1][j + 1]))
                        || (i < grid.len() - 1 && j > 0 && is_symbol(grid[i + 1][j - 1]))
                        || (i < grid.len() - 1 && j < row.len() - 1)
                            && is_symbol(grid[i + 1][j + 1])
                    {
                        symbol_found = true;
                    }
                }
            }

            if symbol_found && ((j == row.len() - 1) || !col.is_digit(10)) {
                total += number;
                number = 0;
                symbol_found = false;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol() {
        assert_eq!(is_symbol('1'), false);
        assert_eq!(is_symbol('.'), false);
        assert_eq!(is_symbol('$'), true);
        assert_eq!(is_symbol('*'), true);
        assert_eq!(is_symbol('#'), true);
    }

    #[test]
    fn test_part_one() {
        let result: Option<usize> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
