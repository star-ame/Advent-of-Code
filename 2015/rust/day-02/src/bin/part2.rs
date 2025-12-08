fn main() {
    let input = include_str!("../../input.txt");

    let result = part2(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part2(input: &str) -> Result<String, String> {
    Ok(input
        .lines()
        .map(|line| {
            let mut nums = line.split('x').map(|n| n.parse::<usize>().unwrap());
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .map(|(a, b, c)| {
            let s1 = a + b;
            let s2 = b + c;
            let s3 = a + c;

            return a * b * c + 2 * [s1, s2, s3].iter().min().unwrap();
        })
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(part2("2x3x4"), Ok("34".to_owned()));
        assert_eq!(part2("1x1x10"), Ok("14".to_owned()));
    }
}
