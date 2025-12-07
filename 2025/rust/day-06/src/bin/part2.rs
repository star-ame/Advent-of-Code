fn main() {
    let input = include_str!("../../input.txt");

    let result = part2(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part2(input: &str) -> Result<String, String> {
    let lines = input.lines().collect::<Vec<_>>();
    let (ops, num_lines) = lines.split_last().unwrap();
    let line_size = num_lines.iter().map(|line| line.len()).max().unwrap();

    let mut full_sum = 0;
    let mut nums = vec![0; 4];
    let mut op = ' ';
    let mut block_start = 0;
    for i in 0..line_size {
        let mut has_digit = false;
        if ops.as_bytes().get(i) == Some(&b'*') {
            op = '*';
        }
        if ops.as_bytes().get(i) == Some(&b'+') {
            op = '+';
        }
        for line in num_lines.iter() {
            let bytes = line.as_bytes();

            let byte = bytes.get(i).unwrap_or(&b' ');

            if byte.is_ascii_digit() {
                has_digit = true;
                nums[i - block_start] =
                    nums[i - block_start] * 10 + (*byte as char).to_digit(10).unwrap() as usize
            }
        }
        if !has_digit || i == line_size - 1 {
            println!("{}, {:?}", op, &nums);
            if op == '*' {
                println!("{}", nums.iter().filter(|n| **n != 0).product::<usize>());
                full_sum += nums.iter().filter(|n| **n != 0).product::<usize>()
            }
            if op == '+' {
                println!("{}", nums.iter().filter(|n| **n != 0).sum::<usize>());
                full_sum += nums.iter().filter(|n| **n != 0).sum::<usize>()
            }
            nums.fill(0);
            block_start = i + 1;
        }
    }

    Ok(full_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
                "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "
            ),
            Ok("3263827".to_owned())
        );
    }
}
