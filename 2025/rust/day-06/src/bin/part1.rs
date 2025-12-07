fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<String, String> {
    let lines = input.lines().collect::<Vec<_>>();

    let iters = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|subs| subs.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    Ok(lines
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .map(|(i, op)| {
            let nums = iters.iter().map(|t| t[i]);

            if op == "*" {
                nums.product::<usize>()
            } else {
                nums.sum::<usize>()
            }
        })
        .sum::<usize>()
        .to_string())

    // let nums = lines
    //     .take_while(|line| line.starts_with(|c: char| c.is_digit(10)))
    //     .map(|line| {
    //         line.split_whitespace()
    //             .map(|subs| subs.parse::<usize>().unwrap())
    //     });

    // let ops_line = lines.next().unwrap();
    // let ops = ops_line.split_whitespace();

    // .map(|c| -> fn(_) -> _ {
    //     if c == "*" {
    //         return Iterator::product;
    //     }
    //     if c == "+" {
    //         return Iterator::sum;
    //     }
    //     panic!("invalid operation, must be '*' or '+'");
    // });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "
            ),
            Ok("4277556".to_owned())
        );
    }
}
