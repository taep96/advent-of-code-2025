advent_of_code::solution!(1);

fn parse_input(input: &str) -> Option<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            let direction = line.chars().next()?;
            let steps = line[1..].parse::<i64>().ok()?;

            match direction {
                'L' => Some(-steps),
                'R' => Some(steps),
                _ => None,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut dial_pos: i64 = 50;

    for delta in parse_input(input)? {
        dial_pos = (dial_pos + delta).rem_euclid(100);
        result += (dial_pos == 0) as u64;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut dial_pos: i64 = 50;

    for delta in parse_input(input)? {
        let offset = -((delta < 0) as i64);
        let next_pos = dial_pos + delta;

        let current_tick = (dial_pos + offset).div_euclid(100);
        let next_tick = (next_pos + offset).div_euclid(100);

        result += (next_tick - current_tick).unsigned_abs();
        dial_pos = next_pos;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
