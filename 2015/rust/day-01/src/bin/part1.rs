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
        .chars()
        .map(|c| if c == '(' { 1isize } else { -1isize })
        .sum::<isize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(part1("(())"), Ok("0".to_owned()));
        assert_eq!(part1("))((((("), Ok("3".to_owned()));
        assert_eq!(part1(")())())"), Ok("-3".to_owned()));
    }
}
