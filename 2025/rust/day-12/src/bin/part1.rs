fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<String, String> {
    let lines = input.lines();
    let a = lines.skip_while(|s| !s.contains('x'));

    let res = a
        .map(|line| line.split_once(": ").unwrap())
        .map(|(region, quantities)| {
            let (x, y) = region.split_once('x').unwrap();
            let (x, y) = (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap());
            let qts = quantities
                .split_whitespace()
                .map(|q| q.parse::<u32>().unwrap());

            let all_qts = qts.sum::<u32>();

            let area = x * y;

            if area < 7 * all_qts {
                0
            } else if area > 9 * all_qts {
                1
            } else {
                1
            }
        })
        .sum::<u32>();

    return Ok(res.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
            ),
            Ok("2".to_owned())
        );
    }
}
