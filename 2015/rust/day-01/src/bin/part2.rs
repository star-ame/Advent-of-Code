use std::ops::Add;

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
        .chars()
        .map(|c| if c == '(' { 1isize } else { -1isize })
        .scan(0, |acc, c| {
            if *acc < 0 {
                return None;
            }
            *acc += c;
            Some(c)
        })
        .enumerate()
        .last()
        .unwrap()
        .0
        .add(1)
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(part2(")"), Ok("1".to_owned()));
        assert_eq!(part2("()())"), Ok("5".to_owned()));
    }
}
