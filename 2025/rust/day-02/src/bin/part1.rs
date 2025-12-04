fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<String, String> {
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

        start_range.filter_map(move |i| {
            let candidate = i.to_string().repeat(2).parse::<u128>().unwrap();
            if (low_num..=high_num).contains(&candidate) {
                return Some(candidate);
            }
            None
        })
    });

    Ok(nums.sum::<u128>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            Ok("1227775554".to_owned())
        );
    }
}
