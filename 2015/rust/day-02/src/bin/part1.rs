fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<String, String> {
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
            let s1 = a * b;
            let s2 = b * c;
            let s3 = a * c;

            return 2 * (s1 + s2 + s3) + [s1, s2, s3].iter().min().unwrap();
        })
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(part1("2x3x4"), Ok("58".to_owned()));
        assert_eq!(part1("2x3x4"), Ok("58".to_owned()));
    }
}
