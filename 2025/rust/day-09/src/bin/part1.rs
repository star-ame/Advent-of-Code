fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<String, String> {
    let coords: Vec<[usize; 2]> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(',').map(|n| n.parse::<usize>().unwrap());
            [iter.next().unwrap(), iter.next().unwrap()]
        })
        .collect();

    Ok(coords
        .iter()
        .flat_map(|coord| coords.iter().map(|other| area(coord, other)))
        .max()
        .unwrap()
        .to_string())
}

fn area(point1: &[usize; 2], point2: &[usize; 2]) -> usize {
    point1
        .iter()
        .zip(point2)
        .map(|(a, b)| a.abs_diff(*b) + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
            ),
            Ok("24".to_owned())
        );
    }
}
