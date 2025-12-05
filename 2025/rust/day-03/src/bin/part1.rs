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
            let mut biggest = '0';
            let mut biggest_idx = 0;
            let mut biggest_2 = '0';
            let mut biggest_2_idx = 0;

            for (i, c) in line.chars().enumerate() {
                if c > biggest {
                    biggest_2 = biggest;
                    biggest_2_idx = biggest_idx;
                    biggest = c;
                    biggest_idx = i;
                }
            }

            if biggest_idx == line.len() - 1 {
                biggest = biggest_2;
                biggest_idx = biggest_2_idx;
            }

            let mut s_biggest = '0';

            for c in line[biggest_idx + 1..].chars() {
                if c > s_biggest {
                    s_biggest = c;
                }
            }

            dbg!(biggest, s_biggest);

            format!("{}{}", biggest, s_biggest)
                .parse::<u32>()
                .expect("biggets and s_biggest will always be a digit")
        })
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            Ok("357".to_owned())
        );
    }
}
