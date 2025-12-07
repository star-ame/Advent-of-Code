use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<String, String> {
    let line_size = input.find('\n').unwrap();
    let mut beams: VecDeque<usize> = VecDeque::new();
    let mut beam_positions: HashSet<usize> = HashSet::new();
    let start = input.find('S').unwrap();
    beams.push_back(start);
    beam_positions.insert(start);

    let mut split_count = 0;
    while let Some(beam) = beams.pop_front() {
        let bottom_pos = beam + line_size + 1;
        if bottom_pos > input.len() {
            break;
        }
        let char = input.as_bytes()[bottom_pos] as char;
        match char {
            '.' => {
                if !beam_positions.contains(&bottom_pos) {
                    beams.push_back(bottom_pos);
                    beam_positions.insert(bottom_pos);
                }
            }
            '^' => {
                split_count += 1;
                let right = bottom_pos + 1;
                let left = bottom_pos - 1;
                if !beam_positions.contains(&right) {
                    beams.push_back(right);
                    beam_positions.insert(right);
                }
                if !beam_positions.contains(&left) {
                    beams.push_back(left);
                    beam_positions.insert(left);
                }
            }
            invalid => {
                return Err(format!("Invalid character: {}", invalid));
            }
        }
    }

    Ok(split_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
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
            Ok("21".to_owned())
        );
    }

    #[test]
    fn minimum() {
        assert_eq!(
            part1(
                ".S.
...
.^.
..."
            ),
            Ok("1".to_owned())
        );
    }

    #[test]
    fn merge() {
        assert_eq!(
            part1(
                "..S..
.....
..^..
.....
.^.^.
....."
            ),
            Ok("3".to_owned())
        );
    }
}
