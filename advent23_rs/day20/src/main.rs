use anyhow::Result;
use lazy_static::lazy_static;
use num::integer::lcm;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

struct PulsePacket {
    source: Box<str>,
    destination: Box<str>,
    pulse: Pulse,
}

#[derive(Clone)]
enum Module {
    FlipFlop {
        name: Box<str>,
        on: bool,
        successors: Vec<Box<str>>,
    },
    Conjunction {
        // most_recent_pulses: HashMap<&'a str, Pulse>,
        name: Box<str>,
        predecessors: Vec<Box<str>>,
        most_recent_pulses: Vec<Pulse>,
        successors: Vec<Box<str>>,
    },
    Broadcaster(Vec<Box<str>>),
    Button,
}

impl Module {
    fn new_flip_flop<'a>(name: &str, successors: impl Iterator<Item = &'a str>) -> Self {
        Module::FlipFlop {
            name: name.into(),
            successors: successors.map(|succ| succ.into()).collect(),

            on: false,
        }
    }

    fn new_conjunction<'a>(name: &str, successors: impl Iterator<Item = &'a str>) -> Self {
        Module::Conjunction {
            name: name.into(),
            successors: successors.map(|succ| succ.into()).collect(),

            predecessors: Vec::new(),
            most_recent_pulses: Vec::new(),
        }
    }

    fn new_broadcaster<'a>(successors: impl Iterator<Item = &'a str>) -> Self {
        Module::Broadcaster(successors.map(|succ| succ.into()).collect())
    }

    /// Returns a reference to the name field of this `Module`
    fn get_name(&self) -> &str {
        match self {
            Module::FlipFlop { name, .. } => name,
            Module::Conjunction { name, .. } => name,
            Module::Broadcaster(_) => "broadcaster",
            Module::Button => "button",
        }
    }

    fn get_successors(&self) -> &[Box<str>] {
        lazy_static! {
            static ref BUTTON_SUCCESSORS: Box<[Box<str>]> = [Box::from("broadcaster")].into();
        };

        match self {
            Module::FlipFlop { successors, .. } => successors,
            Module::Conjunction { successors, .. } => successors,
            Module::Broadcaster(successors) => successors,
            Module::Button => &BUTTON_SUCCESSORS,
        }
    }

    fn process_pulse(&mut self, source: &str, pulse: Pulse, queue: &mut VecDeque<PulsePacket>) {
        let pulse_to_send = match self {
            Module::FlipFlop { on, .. } => {
                // high pulses are ignored
                if matches!(pulse, Pulse::High) {
                    return;
                }

                *on = !*on;

                if *on {
                    Pulse::High
                } else {
                    Pulse::Low
                }
            }
            Module::Conjunction {
                predecessors,
                most_recent_pulses,
                ..
            } => {
                most_recent_pulses[predecessors
                    .iter()
                    .position(|pred| pred as &str == source)
                    .unwrap()] = pulse;

                if most_recent_pulses
                    .iter()
                    .all(|pulse| matches!(pulse, Pulse::High))
                {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            Module::Broadcaster(_) => pulse,
            Module::Button => Pulse::Low,
        };

        for successor in self.get_successors() {
            queue.push_back(PulsePacket {
                source: self.get_name().into(),
                destination: successor.clone(),
                pulse: pulse_to_send,
            })
        }
    }
}

impl From<&str> for Module {
    fn from(line: &str) -> Self {
        let (module_description, successors_str) = line.split_once(" -> ").unwrap();

        let successors = successors_str.split(", ");

        let (first_char, module_name) = module_description.split_at(1);
        match first_char {
            "%" => Module::new_flip_flop(module_name, successors),
            "&" => Module::new_conjunction(module_name, successors),
            "b" => {
                assert_eq!(
                    module_description, "broadcaster",
                    "Cannot parse \"{}\" into Module",
                    line
                );
                Module::new_broadcaster(successors)
            }
            _ => panic!("Cannot parse \"{}\" into Module", line),
        }
    }
}

fn parse_input(input: &str) -> HashMap<Box<str>, Module> {
    let mut result: HashMap<Box<str>, Module> = input
        .lines()
        .map(Module::from)
        .map(|module| (module.get_name().into(), module))
        .collect();

    // copy over all module names and their successors list
    let modules_copy: Vec<Module> = result.values().cloned().collect::<Vec<_>>();

    for module in modules_copy {
        for successor in module.get_successors() {
            if let Some(Module::Conjunction {
                predecessors,
                most_recent_pulses,
                ..
            }) = result.get_mut(successor)
            {
                predecessors.push(module.get_name().into());
                most_recent_pulses.push(Pulse::Low);
            }
        }
    }

    result.insert("button".into(), Module::Button);

    result
}

fn part1(mut graph: HashMap<Box<str>, Module>) -> usize {
    let mut num_low = 0;
    let mut num_high = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::from([PulsePacket {
            source: Box::from("button"),
            destination: Box::from("broadcaster"),
            pulse: Pulse::Low,
        }]);

        while let Some(PulsePacket {
            source,
            destination,
            pulse,
        }) = queue.pop_front()
        {
            match pulse {
                Pulse::Low => num_low += 1,
                Pulse::High => num_high += 1,
            }

            if let Some(module) = graph.get_mut(&destination) {
                module.process_pulse(&source, pulse, &mut queue);
            }
        }
    }

    num_low * num_high
}

fn _part2_brute_force(mut graph: HashMap<Box<str>, Module>) -> usize {
    let mut num_button_presses = 0;

    loop {
        num_button_presses += 1;

        if num_button_presses % 100000 == 0 {
            eprintln!("i = {num_button_presses}");
        }

        let mut queue = VecDeque::from([PulsePacket {
            source: Box::from("button"),
            destination: Box::from("broadcaster"),
            pulse: Pulse::Low,
        }]);

        while let Some(PulsePacket {
            source,
            destination,
            pulse,
        }) = queue.pop_front()
        {
            if matches!(pulse, Pulse::Low) && &destination as &str == "rx" {
                return num_button_presses;
            }

            if let Some(module) = graph.get_mut(&destination) {
                module.process_pulse(&source, pulse, &mut queue);
            }
        }
    }
}

fn part2(mut graph: HashMap<Box<str>, Module>) -> usize {
    let mut num_button_presses = 0;

    // assumptions:
    // 1. fair assumption, &jz -> rx is the only (low) pulse to rx possible
    // 2. All of jz's predecessors run on separate cycles
    // which do not interact with each other, and that jz's predecessors
    // send a high signal and then a low signal once at the end of each cycle,
    // and does not send a different high signal ever,
    // and lastly the cycle starts at the first button press and cleanly resets
    // after the button press which sends a signal to jz

    // for example some output of all signals sent to jz
    // where i is the number of button presses:

    // source = vf, destination = jz, i = 3847, switched to High
    // source = vf, destination = jz, i = 3847, switched to Low
    // source = rn, destination = jz, i = 3923, switched to High
    // source = rn, destination = jz, i = 3923, switched to Low
    // source = dh, destination = jz, i = 4001, switched to High
    // source = dh, destination = jz, i = 4001, switched to Low
    // source = mk, destination = jz, i = 4091, switched to High
    // source = mk, destination = jz, i = 4091, switched to Low
    // source = vf, destination = jz, i = 7694, switched to High
    // source = vf, destination = jz, i = 7694, switched to Low
    // source = rn, destination = jz, i = 7846, switched to High
    // source = rn, destination = jz, i = 7846, switched to Low
    // source = dh, destination = jz, i = 8002, switched to High
    // source = dh, destination = jz, i = 8002, switched to Low
    // source = mk, destination = jz, i = 8182, switched to High
    // source = mk, destination = jz, i = 8182, switched to Low
    // source = vf, destination = jz, i = 11541, switched to High
    // source = vf, destination = jz, i = 11541, switched to Low
    // source = rn, destination = jz, i = 11769, switched to High
    // source = rn, destination = jz, i = 11769, switched to Low
    // source = dh, destination = jz, i = 12003, switched to High
    // source = dh, destination = jz, i = 12003, switched to Low
    // source = mk, destination = jz, i = 12273, switched to High
    // source = mk, destination = jz, i = 12273, switched to Low

    let jz_predecessors = if let Module::Conjunction { predecessors, .. } = &graph["jz"] {
        predecessors.clone()
    } else {
        panic!()
    };
    let mut jz_predecessor_first_button_presses: Vec<Option<usize>> =
        vec![None; jz_predecessors.len()];

    let mut num_jz_pred_found = 0;

    loop {
        num_button_presses += 1;

        let mut queue = VecDeque::from([PulsePacket {
            source: Box::from("button"),
            destination: Box::from("broadcaster"),
            pulse: Pulse::Low,
        }]);

        while let Some(PulsePacket {
            source,
            destination,
            pulse,
        }) = queue.pop_front()
        {
            if matches!(pulse, Pulse::High) && &destination as &str == "jz" {
                let index = jz_predecessors
                    .iter()
                    .position(|pred| pred == &source)
                    .unwrap();

                if jz_predecessor_first_button_presses[index].is_none() {
                    jz_predecessor_first_button_presses[index] = Some(num_button_presses);
                    num_jz_pred_found += 1;
                    if num_jz_pred_found == jz_predecessors.len() {
                        return jz_predecessor_first_button_presses
                            .into_iter()
                            .map(|i| i.unwrap())
                            .reduce(lcm)
                            .unwrap();
                    }
                }
            }

            if let Some(module) = graph.get_mut(&destination) {
                module.process_pulse(&source, pulse, &mut queue);
            }
        }
    }
}

fn main() -> Result<()> {
    let file_contents = fs::read_to_string("input.txt")?;

    let graph = parse_input(&file_contents);

    println!("{}", part1(graph.clone()));
    // println!("{}", _part2_brute_force(graph));
    println!("{}", part2(graph));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = "broadcaster -> a, b, c\n\
                             %a -> b\n\
                             %b -> c\n\
                             %c -> inv\n\
                             &inv -> a";

    const EXAMPLE_B: &str = "broadcaster -> a\n\
                             %a -> inv, con\n\
                             &inv -> b\n\
                             %b -> con\n\
                             &con -> output";

    #[test]
    fn test_part1_a() {
        let graph = parse_input(EXAMPLE_A);

        assert_eq!(32000000, part1(graph));
    }

    #[test]
    fn test_part1_b() {
        let graph = parse_input(EXAMPLE_B);

        assert_eq!(11687500, part1(graph));
    }
}
