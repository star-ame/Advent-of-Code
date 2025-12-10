fn main() {
    let input = include_str!("../../input.txt");

    let result = part2(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn part2(input: &str) -> Result<String, String> {
    let coords: Vec<Point> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(',').map(|n| n.parse::<usize>().unwrap());
            Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            }
        })
        .collect();

    let mut boxes: Vec<(&Point, &Point, usize)> = coords
        .iter()
        .flat_map(|coord| coords.iter().map(move |other_coord| (coord, other_coord)))
        .map(|(a, b)| (a, b, area(a, b)))
        .collect();

    boxes.sort_by_key(|(_, _, area)| *area);

    dbg!(&boxes);

    Ok(boxes
        .iter()
        .rev()
        .find(|(a, b, _)| {
            // println!(
            //     "{} of {} ({}%)",
            //     i + 1,
            //     coords.len().pow(2),
            //     (i + 1) * 100 / coords.len().pow(2)
            // );

            let wrap_around: &[Point] = &[
                coords.last().unwrap().clone(),
                coords.first().unwrap().clone(),
            ][..];

            let is_inside = coords
                .windows(2)
                .chain([wrap_around])
                .any(|pairs| does_line_cut_box(a, b, pairs));

            return !is_inside;
        })
        .unwrap()
        .2
        .to_string())
}

fn does_line_cut_box(box_p1: &&Point, box_p2: &&Point, line: &[Point]) -> bool {
    let c1 = &line[0];
    let c2 = &line[1];

    let min_x = usize::min(box_p1.x, box_p2.x);
    let max_x = usize::max(box_p1.x, box_p2.x);
    let min_y = usize::min(box_p1.y, box_p2.y);
    let max_y = usize::max(box_p1.y, box_p2.y);

    let x_overlap = get_overlap(min_x, max_x, c1.x);
    let y_overlap = get_overlap(min_y, max_y, c1.y);

    match (x_overlap, y_overlap) {
        // obviously beyound the bounds of the box
        (Overlap::Outside(_), Overlap::Outside(_)) => return false,
        // obviously inside the box
        (Overlap::Inside, Overlap::Inside) => return true,
        // we should only really get here when we're comparing a point
        // that is one of the points of the box so we can overlook it
        (Overlap::Edge(_), Overlap::Edge(_)) => {
            return false;
        }
        (Overlap::Outside(c1_dir), Overlap::Inside) => {
            handle_x(c2, min_x, max_x, min_y, max_y, c1_dir)
        }
        (Overlap::Edge(c1_dir), Overlap::Inside) => {
            handle_x(c2, min_x, max_x, min_y, max_y, c1_dir)
        }
        (Overlap::Inside, Overlap::Outside(c1_dir)) => {
            handle_y(c2, min_x, max_x, min_y, max_y, c1_dir)
        }
        (Overlap::Inside, Overlap::Edge(c1_dir)) => {
            handle_y(c2, min_x, max_x, min_y, max_y, c1_dir)
        }
        (Overlap::Outside(_), Overlap::Edge(_)) => {
            return false;
        }
        (Overlap::Edge(_), Overlap::Outside(_)) => {
            return false;
        }
        // special case
        (Overlap::Equal, _) => return false,
        (_, Overlap::Equal) => return false,
    }
}

fn handle_x(
    c2: &Point,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    c1_dir: OverlapDir,
) -> bool {
    let x_overlap = get_overlap(min_x, max_x, c2.x);
    let y_overlap = get_overlap(min_y, max_y, c2.y);
    if y_overlap != Overlap::Inside {
        return false;
    }

    match x_overlap {
        Overlap::Inside => return true,
        Overlap::Edge(other_dir) => return other_dir == c1_dir.other(),
        Overlap::Outside(other_dir) => return other_dir == c1_dir.other(),
        Overlap::Equal => unreachable!("im gonna cry"),
    }
}

fn handle_y(
    c2: &Point,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    c1_dir: OverlapDir,
) -> bool {
    let x_overlap = get_overlap(min_x, max_x, c2.x);
    let y_overlap = get_overlap(min_y, max_y, c2.y);
    if x_overlap != Overlap::Inside {
        return false;
    }

    match y_overlap {
        Overlap::Inside => return true,
        Overlap::Edge(other_dir) => return other_dir == c1_dir.other(),
        Overlap::Outside(other_dir) => return other_dir == c1_dir.other(),
        Overlap::Equal => unreachable!("im gonna cry"),
    }
}

fn get_overlap(start: usize, end: usize, point: usize) -> Overlap {
    if start == end && end == point {
        return Overlap::Equal;
    }
    if point < start {
        return Overlap::Outside(OverlapDir::Start);
    }
    if point == start {
        return Overlap::Edge(OverlapDir::Start);
    }
    if point > start && point < end {
        return Overlap::Inside;
    }
    if point == end {
        return Overlap::Edge(OverlapDir::End);
    }
    if point > end {
        return Overlap::Outside(OverlapDir::End);
    }
    unreachable!()
}

#[derive(PartialEq, Eq)]
enum OverlapDir {
    Start,
    End,
}
impl OverlapDir {
    fn other(&self) -> OverlapDir {
        match self {
            OverlapDir::Start => OverlapDir::End,
            OverlapDir::End => OverlapDir::Start,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Overlap {
    Outside(OverlapDir),
    Edge(OverlapDir),
    Inside,
    Equal,
}

fn area(point1: &Point, point2: &Point) -> usize {
    (point1.x.abs_diff(point2.x) + 1) * (point1.y.abs_diff(point2.y) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
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

    #[test]
    fn hard_one() {
        assert_eq!(
            part2(
                "1,1
1,20
20,20
20,1
18,1
18,18
3,18
3,1"
            ),
            Ok("60".to_owned())
        );
    }

    #[test]
    fn hard_two() {
        assert_eq!(
            part2(
                "1,1
1,3
3,3
3,4
1,4
1,5
5,5
5,1"
            ),
            Ok("25".to_owned())
        );
    }

    #[test]
    fn area_test() {
        assert_eq!(area(&Point { x: 2, y: 3 }, &Point { x: 7, y: 3 }), 6);
        assert_eq!(area(&Point { x: 7, y: 3 }, &Point { x: 2, y: 3 }), 6);
        assert_eq!(area(&Point { x: 7, y: 1 }, &Point { x: 11, y: 7 }), 35);
    }
}
