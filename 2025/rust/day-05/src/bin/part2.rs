fn main() {
    let input = include_str!("../../input.txt");

    let result = part2(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part2(input: &str) -> Result<String, String> {
    let (i_ranges, _) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<Range> = i_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect();

    ranges.sort_by_key(|range| range.start);

    let mut i = 0;
    loop {
        if i >= ranges.len() - 1 {
            break;
        }

        let range_r = ranges[i + 1].clone();
        let range_l = ranges.get_mut(i).unwrap();

        if range_r.start <= range_l.end + 1 {
            let new_end = range_r.end.clone();

            range_l.end = new_end.max(range_l.end);
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }

    Ok(ranges
        .iter()
        .map(|range| range.end - range.start + 1)
        .sum::<usize>()
        .to_string())
}

#[derive(Clone)]
struct Range {
    start: usize,
    end: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
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
            Ok("14".to_owned())
        );
    }

    #[test]
    fn inside() {
        assert_eq!(
            part2(
                "5-10
7-9

1"
            ),
            Ok("6".to_owned())
        );
    }

    #[test]
    fn adjacent() {
        assert_eq!(
            part2(
                "5-10
11-14

1"
            ),
            Ok("10".to_owned())
        );
    }
}
