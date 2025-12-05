use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");

    let result = part2(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part2(input: &str) -> Result<String, String> {
    let ranges = input
        .split(',')
        .map(|s| s.split_once('-').unwrap())
        .collect::<Vec<_>>();

    let nums = ranges.iter().flat_map(|range| {
        let low_str = range.0;
        let low_num = low_str.parse::<u128>().unwrap();
        let low_start = &low_str[..(low_str.len() / 2).max(1)];

        let high_str = range.1;
        let high_num = high_str.parse::<u128>().unwrap();
        let high_start = &high_str[..(high_str.len() as f32 / 2.).round().max(1.) as usize];

        let start_range = low_start.parse::<u128>().unwrap()..=high_start.parse::<u128>().unwrap();

        HashSet::<_>::from_iter(start_range.flat_map(move |i| {
            let i_str = i.to_string();
            (1..=i_str.len()).flat_map(move |size| {
                (low_str.len().max(2)..=high_str.len().max(2))
                    .map({
                        let value = i_str.clone();
                        move |final_size| {
                            value[..size]
                                .repeat(final_size / size)
                                .parse::<u128>()
                                .unwrap()
                        }
                    })
                    .filter(move |num| (low_num..=high_num).contains(num))
            })
        }))
    });

    Ok(nums.sum::<u128>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            Ok("4174379265".to_owned())
        );
    }

    #[test]
    fn one() {
        assert_eq!(part2("1-20"), Ok("11".to_owned()));
    }
}
