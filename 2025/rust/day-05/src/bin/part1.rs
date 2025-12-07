use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<String, String> {
    let (i_ranges, i_ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<RangeInclusive<usize>> = i_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let ids = i_ids.lines().map(|line| line.parse().unwrap());

    return Ok(ids
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
        .to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            Ok("3".to_owned())
        );
    }
}
