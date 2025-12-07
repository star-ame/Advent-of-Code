use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../input.txt");

    let result = part2(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part2(input: &str) -> Result<String, String> {
    Ok(input.lines().map(get_joltage).sum::<u64>().to_string())
}

fn get_joltage(line: &str) -> u64 {
    let range = (line.len() - 12)..line.len();
    let mut ptrs = range.collect::<Vec<_>>();

    let mut digit_map = line.chars().take(line.len() - 12).enumerate().fold(
        // this vec's 0th position is unused since input digits go 1-9
        // we could make the vec go from 0 to 8 instead,
        // with 0 representing 1, up to 8 representing 9, but
        // for the sake of readability we'll just have 10 spots instead
        // with each index representing it's number, 0 unused
        vec![VecDeque::with_capacity(line.len()); 10],
        |mut vec, (i, c)| {
            vec.get_mut(c.to_digit(10).expect("invalid input") as usize)
                .unwrap()
                .push_back(i);
            return vec;
        },
    );

    let mut min_ptr = 0;

    for curr_ptr_idx in 0..ptrs.len() {
        let curr_ptr = ptrs[curr_ptr_idx];
        if curr_ptr_idx > 0 {
            let prev_ptr = ptrs[curr_ptr_idx - 1];
            if prev_ptr == curr_ptr - 1 {
                break;
            }
        }

        let curr_digit = line[curr_ptr..]
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;

        for big_digit in (curr_digit..=9).rev() {
            match digit_map[big_digit].pop_front() {
                Some(big_digit_ptr) => {
                    ptrs[curr_ptr_idx] = big_digit_ptr;
                    // remove chars to the left of the new leftmost digit
                    // from the digit_map
                    line[min_ptr..big_digit_ptr].chars().for_each(|digit| {
                        digit_map[digit.to_digit(10).unwrap() as usize].pop_front();
                    });
                    min_ptr = big_digit_ptr + 1;

                    digit_map[curr_digit].push_back(curr_ptr);
                    break;
                }
                None => {
                    continue;
                }
            }
        }
    }

    ptrs.iter()
        .map(|ptr| line[*ptr..].chars().next().unwrap())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
                "987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            Ok("3121910778619".to_owned())
        );
    }
    #[test]
    fn one() {
        assert_eq!(part2("987654321111111"), Ok("987654321111".to_owned()));
    }
    #[test]
    fn two() {
        assert_eq!(part2("811111111111119"), Ok("811111111119".to_owned()));
    }
    #[test]
    fn three() {
        assert_eq!(part2("234234234234278"), Ok("434234234278".to_owned()));
    }
    #[test]
    fn four() {
        assert_eq!(part2("818181911112111"), Ok("888911112111".to_owned()));
    }

    #[test]
    fn five() {
        assert_eq!(part2("5631465827645468355653355523535772463346671437436425546664765663657756643567465459577656563565556374"), Ok("977666656374".to_owned()));
    }
}
