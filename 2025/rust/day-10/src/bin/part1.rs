use std::collections::HashSet;

use nom::{
    bytes::tag,
    character::streaming::digit1,
    combinator::map_res,
    multi::{self, many1, separated_list1},
    sequence::delimited,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    match result {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

fn part1(input: &str) -> Result<String, String> {
    let machines: Vec<_> = input.lines().map(|line| machine(line).unwrap().1).collect();
    // let (_, machines) = separated_list1(newline, machine)
    //     .parse(input)
    //     .map_err(|e| format!("{}", e))?;

    Ok(machines
        .iter()
        .map(|machine| fewest_presses(machine))
        .sum::<usize>()
        .to_string())
}

fn fewest_presses(machine: &Machine) -> usize {
    let options: Vec<ButtonCombo> = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(i, btn)| ButtonCombo {
            btns: HashSet::from_iter([i]),
            light_toggles: btn.light_toggles.clone(),
            presses: 1,
        })
        .collect();

    return fewest_presses_rec(machine, options);
}

fn fewest_presses_rec(machine: &Machine, options: Vec<ButtonCombo>) -> usize {
    if options.len() == 0 {
        panic!("Infinite recursion")
    }
    let correct = options.iter().find(|opt| {
        machine
            .lights
            .toggle(&opt.light_toggles)
            .iter()
            .all(|light| *light == Light::Off)
    });

    if let Some(correct) = correct {
        println!("FINISHED --------------------------------");
        return correct.presses;
    }

    let new_options: Vec<_> = options
        .iter()
        .flat_map(|combo| {
            machine.buttons.iter().enumerate().filter_map(|(i, btn)| {
                if combo.btns.contains(&i) {
                    return None;
                }
                let diff: HashSet<usize> = combo
                    .light_toggles
                    .symmetric_difference(&btn.light_toggles)
                    .map(|i| i.to_owned())
                    .collect();
                if diff.is_empty() {
                    return None;
                }
                let mut my_btns = combo.btns.clone();
                my_btns.insert(i);
                Some(ButtonCombo {
                    btns: my_btns,
                    light_toggles: diff,
                    presses: combo.presses + 1,
                })
            })
        })
        .collect();

    // dbg!(&new_options);

    return fewest_presses_rec(machine, new_options);
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, ls) = lights(input)?;
    let (input, _) = tag(" ").parse(input)?;
    let (input, btns) = buttons(input)?;
    let (input, _) = tag(" ").parse(input)?;
    let (input, jolts) = joltage(input)?;

    Ok((
        input,
        Machine {
            buttons: btns,
            joltage: jolts,
            lights: ls,
        },
    ))
}

fn lights(input: &str) -> IResult<&str, Vec<Light>> {
    delimited(tag("["), many1(light), tag("]")).parse(input)
}

fn light(input: &str) -> IResult<&str, Light> {
    let (input, l) = tag(".").or(tag("#")).parse(input)?;

    if l == "." {
        Ok((input, Light::Off))
    } else {
        Ok((input, Light::On))
    }
}

fn buttons(input: &str) -> IResult<&str, Vec<Button>> {
    separated_list1(tag(" "), button).parse(input)
}

fn button(input: &str) -> IResult<&str, Button> {
    let light_positions = multi::separated_list1(tag(","), map_res(digit1, str::parse));
    let (input, light_positions) = delimited(tag("("), light_positions, tag(")")).parse(input)?;

    Ok((
        input,
        Button {
            light_toggles: HashSet::from_iter(light_positions),
        },
    ))
}

fn joltage(input: &str) -> IResult<&str, Vec<usize>> {
    let levels = multi::separated_list1(tag(","), map_res(digit1, str::parse));

    let (input, light_positions) = delimited(tag("{"), levels, tag("}")).parse(input)?;

    Ok((input, light_positions))
}

#[derive(Clone, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

impl Light {
    fn toggle(&self) -> Light {
        match self {
            Light::On => Light::Off,
            Light::Off => Light::On,
        }
    }
}

trait Toggle {
    fn toggle(&self, which: &HashSet<usize>) -> Self;
}
impl Toggle for Vec<Light> {
    fn toggle(&self, which: &HashSet<usize>) -> Self {
        self.iter()
            .enumerate()
            .map(|(i, l)| {
                if which.contains(&i) {
                    l.toggle()
                } else {
                    l.clone()
                }
            })
            .collect()
    }
}

struct Button {
    light_toggles: HashSet<usize>,
}

struct Machine {
    lights: Vec<Light>,
    buttons: Vec<Button>,
    #[allow(unused)]
    joltage: Vec<usize>,
}

#[derive(Debug)]
struct ButtonCombo {
    light_toggles: HashSet<usize>,
    presses: usize,
    btns: HashSet<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            ),
            Ok("7".to_owned())
        );
    }
}
