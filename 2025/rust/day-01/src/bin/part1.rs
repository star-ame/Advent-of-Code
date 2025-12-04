fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<i16, String> {
    let parsed = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.try_into())
        .collect::<Result<Vec<_>, RotationError>>();

    let Ok(parsed) = parsed else {
        return Err(format!("failed to parse: {:?}", parsed.unwrap_err().0));
    };

    Ok(parsed
        .iter()
        .fold(
            State {
                cur_pos: 50,
                zero_count: 0,
            },
            |state, item| {
                let res = match item {
                    Rotation::Left(amt) => {
                        let new_pos = (state.cur_pos - *amt).rem_euclid(100) % 100;
                        let new_count = if new_pos == 0 {
                            state.zero_count + 1
                        } else {
                            state.zero_count
                        };
                        State {
                            cur_pos: new_pos,
                            zero_count: new_count,
                        }
                    }
                    Rotation::Right(amt) => {
                        let new_pos = (state.cur_pos + *amt) % 100;
                        let new_count = if new_pos == 0 {
                            state.zero_count + 1
                        } else {
                            state.zero_count
                        };
                        State {
                            cur_pos: new_pos,
                            zero_count: new_count,
                        }
                    }
                };
                return res;
            },
        )
        .zero_count)
}

#[derive(Debug)]
struct State {
    cur_pos: i16,
    zero_count: i16,
}

#[derive(Debug)]
enum Rotation {
    Left(i16),
    Right(i16),
}

#[derive(Debug)]
struct RotationError<'a>(&'a str);

impl<'a> TryFrom<&'a str> for Rotation {
    type Error = RotationError<'a>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.starts_with('R') {
            let amount = i16::from_str_radix(&value[1..], 10).map_err(|_| RotationError(value))?;
            return Ok(Rotation::Right(amount));
        }
        if value.starts_with('L') {
            let amount = i16::from_str_radix(&value[1..], 10).map_err(|_| RotationError(value))?;
            return Ok(Rotation::Left(amount));
        }
        return Err(RotationError(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            ),
            Ok(3)
        );
    }
}
