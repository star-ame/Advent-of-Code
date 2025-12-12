use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../../input.txt");

    let result = part2(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part2(input: &str) -> Result<String, String> {
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
            source_node.i_reference = destinations.clone();

            destinations.iter().for_each(|destination| {
                let dest_node = nodes.entry(destination).or_insert_with(Node::default);

                dest_node.references_me.push(source);
            });
        });

    paint_fft(&mut nodes);
    paint_dac(&mut nodes);

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
            if source_node.part_of_fft_chain && source_node.part_of_dac_chain {
                println!("ok: {}", source_str);
                source_node.count_to_out += dest_count;
            } else {
                println!("no: {}", source_str);
            }

            if source_node.outgoing_references == 0 {
                queue.push_back(source_str);
            }
        }
    }

    Ok(nodes.get("svr").unwrap().count_to_out.to_string())
}

fn paint_fft(nodes: &mut HashMap<&str, Node<'_>>) {
    let mut fft_queue = VecDeque::from(["fft"]);
    while let Some(fft_key) = fft_queue.pop_front() {
        let fft_node = nodes.get_mut(fft_key).unwrap();
        fft_node.part_of_fft_chain = true;
        for fwd_ref in &fft_node.i_reference {
            fft_queue.push_back(fwd_ref);
        }
    }
    fft_queue.push_back("fft");
    while let Some(fft_key) = fft_queue.pop_front() {
        let fft_node = nodes.get_mut(fft_key).unwrap();
        fft_node.part_of_fft_chain = true;
        for bwd_ref in &fft_node.references_me {
            fft_queue.push_back(bwd_ref);
        }
    }
}

fn paint_dac(nodes: &mut HashMap<&str, Node<'_>>) {
    let mut dac_queue = VecDeque::from(["dac"]);
    while let Some(dac_key) = dac_queue.pop_front() {
        let dac_node = nodes.get_mut(dac_key).unwrap();
        dac_node.part_of_dac_chain = true;
        for fwd_ref in &dac_node.i_reference {
            dac_queue.push_back(fwd_ref);
        }
    }
    dac_queue.push_back("dac");
    while let Some(dac_key) = dac_queue.pop_front() {
        let dac_node = nodes.get_mut(dac_key).unwrap();
        dac_node.part_of_dac_chain = true;
        for fwd_ref in &dac_node.references_me {
            dac_queue.push_back(fwd_ref);
        }
    }
}

#[derive(Debug, Default)]
struct Node<'a> {
    references_me: Vec<&'a str>,
    i_reference: Vec<&'a str>,
    outgoing_references: usize,
    count_to_out: usize,
    part_of_fft_chain: bool,
    part_of_dac_chain: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
                "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
            ),
            Ok("2".to_owned())
        );
    }
}
