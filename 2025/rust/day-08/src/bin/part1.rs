use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input, 1000);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str, connections: usize) -> Result<String, String> {
    let coords: Vec<[f32; 3]> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(',').map(|n| n.parse::<f32>().unwrap());
            [
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]
        })
        .collect();

    let mut nearests: Vec<(usize, usize, f32)> = coords
        .iter()
        .enumerate()
        .flat_map(|(i, coord)| {
            let others = coords
                .iter()
                .enumerate()
                .filter(move |(j, _)| *j != i)
                .map(move |(j, other)| (i.min(j), i.max(j), distance(coord, other)));

            others
        })
        .collect();

    nearests.sort_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2));
    nearests.dedup();

    let d = nearests
        .iter()
        .map(|(a, b, d)| {
            (
                coords[*a].map(|f| f.to_string()).join(","),
                coords[*b].map(|f| f.to_string()).join(","),
                d,
            )
        })
        .collect::<Vec<_>>();

    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for (p1, p2, _) in nearests.iter().take(connections) {
        let mut found_circuit = None;
        let mut i = 0;
        while i < circuits.len() {
            let circuit = &mut circuits[i];

            let hasp1 = circuit.contains(&p1);
            let hasp2 = circuit.contains(&p2);

            if !(hasp1 || hasp2) {
                i += 1;
                continue;
            }

            // merge circuits when both a and b are part of a circuit
            // but those circuits are not the same
            if let Some(found_before_pos) = found_circuit {
                let circuit = circuits.swap_remove(i);
                let actual_circuit: &HashSet<usize> = &circuits[found_before_pos];
                let intersection = actual_circuit
                    .union(&circuit)
                    .map(|i| i.to_owned())
                    .collect::<HashSet<_>>();
                circuits[found_before_pos] = intersection;
                break;
            }

            if hasp1 && hasp2 {
                found_circuit = Some(i);
                break;
            }

            if hasp1 {
                circuit.insert(*p2);
                found_circuit = Some(i);
            }
            if hasp2 {
                circuit.insert(*p1);
                found_circuit = Some(i);
            }
            i += 1;
        }

        if found_circuit.is_none() {
            let mut set = HashSet::new();
            set.insert(*p1);
            set.insert(*p2);
            circuits.push(set)
        }
    }

    circuits.sort_by_key(|circuit| circuit.len());

    let result = circuits
        .last_chunk::<3>()
        .map(|chunk| chunk.iter().map(|set| set.len()).product::<usize>())
        .unwrap_or_else(|| circuits.iter().map(|set| set.len()).product::<usize>());

    Ok(result.to_string())
}

fn distance(point1: &[f32; 3], point2: &[f32; 3]) -> f32 {
    point1
        .iter()
        .zip(point2)
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
                10
            ),
            Ok("40".to_owned())
        );
    }
}
