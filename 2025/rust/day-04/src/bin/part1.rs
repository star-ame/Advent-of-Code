fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<String, String> {
    let input_size = input.len();
    let row_size = input.lines().next().unwrap().len();
    let row_count = input_size / row_size;

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
                            continue 'cells;
                        }
                    }
                }
            }
            counter += 1;
        }
    }

    Ok(counter.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
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
            Ok("13".to_owned())
        );
    }
}
