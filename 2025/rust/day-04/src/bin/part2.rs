fn main() {
    let input = include_str!("../../input.txt");

    let result = part2(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part2(input: &str) -> Result<String, String> {
    part2_rec(input, 0).map(|res| res.to_string())
}

fn part2_rec(input: &str, global_counter: usize) -> Result<usize, String> {
    let input_size = input.len();
    let row_size = input.lines().next().unwrap().len();
    let row_count = input_size / row_size;

    let mut next_input = String::with_capacity(input_size);

    let get = |(x, y): (usize, usize)| {
        if x >= row_size {
            return None;
        }
        if y >= row_count {
            return None;
        }

        Some(input.as_bytes()[y * (row_size + 1) + x] as char)
    };

    let mut counter = 0;

    for y in 0..row_count {
        'cells: for x in 0..row_size {
            let c = get((x, y));
            if c == Some('.') {
                next_input.push('.');
                continue;
            }

            let mut count = 0;
            for y_offset in -1isize..=1 {
                for x_offset in -1isize..=1 {
                    if y_offset == 0 && x_offset == 0 {
                        continue;
                    }
                    let Some(n_x) = x.checked_add_signed(x_offset) else {
                        continue;
                    };
                    let Some(n_y) = y.checked_add_signed(y_offset) else {
                        continue;
                    };
                    let c = get((n_x, n_y));
                    if c == Some('@') {
                        count += 1;
                        if count >= 4 {
                            // can't remove
                            next_input.push('@');
                            continue 'cells;
                        }
                    }
                }
            }
            // removed roll
            next_input.push('.');
            counter += 1;
        }
        next_input.push('\n');
    }

    if counter == 0 {
        return Ok(global_counter);
    }
    return part2_rec(&next_input.trim_end(), global_counter + counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            ),
            Ok("43".to_owned())
        );
    }
}
