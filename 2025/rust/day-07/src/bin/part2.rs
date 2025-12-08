use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");

    let result = part2(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part2(input: &str) -> Result<String, String> {
    let line_size = input.find('\n').unwrap() + 1;
    let start = input.find('S').unwrap();

    let mut hashmap = HashMap::new();
    Ok(path_count(input, line_size, &mut hashmap, start).to_string())
}

fn path_count(
    input: &str,
    line_size: usize,
    pos_path_count_map: &mut HashMap<usize, usize>,
    tachion_pos: usize,
) -> usize {
    if let Some(result) = pos_path_count_map.get(&tachion_pos) {
        return *result;
    }
    let bottom_pos = tachion_pos + line_size * 2;

    if bottom_pos > input.len() {
        pos_path_count_map.insert(tachion_pos, 1);
        return 1;
    }

    let char = input.as_bytes()[bottom_pos] as char;
    match char {
        '.' => {
            let result = path_count(input, line_size, pos_path_count_map, bottom_pos);
            pos_path_count_map.insert(tachion_pos, result);
            return result;
        }
        '^' => {
            let right = bottom_pos + 1;
            let left = bottom_pos - 1;
            let result = path_count(input, line_size, pos_path_count_map, left)
                + path_count(input, line_size, pos_path_count_map, right);
            pos_path_count_map.insert(tachion_pos, result);
            return result;
        }
        invalid => {
            Err::<(), String>(format!("Invalid character: {}", invalid)).unwrap();
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
                ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            ),
            Ok("40".to_owned())
        );
    }

    #[test]
    fn minimum() {
        assert_eq!(
            part2(
                ".S.
...
.^.
..."
            ),
            Ok("2".to_owned())
        );
    }

    #[test]
    fn merge() {
        assert_eq!(
            part2(
                "..S..
.....
..^..
.....
.^.^.
....."
            ),
            Ok("4".to_owned())
        );
    }

    #[test]
    fn merge_more() {
        assert_eq!(
            part2(
                "....S....
.........
....^....
.........
...^.^...
.........
..^.^.^..
........."
            ),
            Ok("8".to_owned())
        );
    }
}
