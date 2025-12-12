use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<String, String> {
    // this is actually a map of destinations to sources
    let mut nodes = HashMap::<&str, Node>::new();

    input
        .lines()
        .map(|line| {
            let (name, connections) = line.split_once(": ").unwrap();

            let connections = connections.split_whitespace().collect::<Vec<_>>();

            return (name, connections);
        })
        .for_each(|(source, destinations)| {
            let source_node = nodes.entry(source).or_insert_with(Node::default);
            source_node.outgoing_references = destinations.len();
            destinations.iter().for_each(|destination| {
                let dest_node = nodes.entry(destination).or_insert_with(Node::default);

                dest_node.references_me.push(source);
            })
        });
    dbg!(&nodes);

    let out = nodes.get_mut("out").unwrap();
    out.count_to_out = 1;

    let mut queue = VecDeque::from(["out"]);

    while let Some(item) = queue.pop_front() {
        let dest_node = nodes.get(item).unwrap();
        let dest_count = dest_node.count_to_out;

        for source_str in dest_node.references_me.clone() {
            let Some(source_node) = nodes.get_mut(source_str) else {
                continue;
            };
            // println!("{} -> {}", source_str, item);
            source_node.outgoing_references -= 1;
            source_node.count_to_out += dest_count;

            if source_node.outgoing_references == 0 {
                queue.push_back(source_str);
            }
        }
    }

    Ok(nodes.get("you").unwrap().count_to_out.to_string())
}

#[derive(Debug, Default)]
struct Node<'a> {
    references_me: Vec<&'a str>,
    outgoing_references: usize,
    count_to_out: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
            ),
            Ok("5".to_owned())
        );
    }
}
