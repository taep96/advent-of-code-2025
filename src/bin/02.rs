advent_of_code::solution!(2);

fn parse_input(input: &str) -> impl Iterator<Item = std::ops::RangeInclusive<u64>> {
    input.trim().split(',').filter_map(|range| {
        let (a, b) = range.split_once('-')?;
        Some(a.parse().ok()?..=b.parse().ok()?)
    })
}

fn num_digits(n: u64) -> usize {
    if n == 0 { 1 } else { (n.ilog10() + 1) as usize }
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = parse_input(input)
        .map(|range| {
            let start = *range.start();
            let end = *range.end();
            let min_digits = num_digits(start);
            let max_digits = num_digits(end);

            let mut total = 0u64;

            for digits in (2..=max_digits).step_by(2) {
                if digits < min_digits {
                    continue;
                }

                let half = digits / 2;
                let half_pow = 10u64.pow(half as u32);

                let min_half_value = 10u64.pow((half - 1) as u32);
                let max_half_value = half_pow - 1;

                let multiplier = half_pow + 1;

                let lower_bound = if digits == min_digits {
                    min_half_value.max(start.div_ceil(multiplier))
                } else {
                    min_half_value
                };

                let upper_bound = if digits == max_digits {
                    max_half_value.min(end / multiplier)
                } else {
                    max_half_value
                };

                if lower_bound <= upper_bound {
                    let count = upper_bound - lower_bound + 1;
                    total += multiplier * count * (lower_bound + upper_bound) / 2;
                }
            }

            total
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = parse_input(input)
        .map(|range| {
            let start = *range.start();
            let end = *range.end();
            let min_digits = num_digits(start);
            let max_digits = num_digits(end);

            let mut pow10 = vec![1u64; max_digits + 1];
            for i in 1..=max_digits {
                pow10[i] = pow10[i - 1] * 10;
            }

            let mut seen = std::collections::HashSet::new();
            let mut total = 0u64;

            for pattern_len in 1..=max_digits {
                let base = pow10[pattern_len];
                let min_pattern = pow10[pattern_len - 1];
                let max_pattern = base - 1;

                for repetitions in 2..=(max_digits / pattern_len) {
                    let digits = pattern_len * repetitions;
                    if digits < min_digits || digits > max_digits {
                        continue;
                    }

                    let multiplier =
                        (0..repetitions).fold(0u64, |acc, i| acc + pow10[pattern_len * i]);

                    let lower_bound = if digits == min_digits {
                        min_pattern.max(start.div_ceil(multiplier))
                    } else {
                        min_pattern
                    };

                    let upper_bound = if digits == max_digits {
                        max_pattern.min(end / multiplier)
                    } else {
                        max_pattern
                    };

                    for pattern in lower_bound..=upper_bound {
                        let n = pattern * multiplier;
                        if seen.insert(n) {
                            total += n;
                        }
                    }
                }
            }
            total
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
