use anyhow::Result;
use num::integer::lcm;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs,
    ops::{Deref, DerefMut},
};

mod utils;
use utils::{Broadcaster, Button, Conjunction, FlipFlop, Module, ModuleEnum, Pulse, PulsePacket};

enum ModuleOrPlaceholder<'a> {
    Module(ModuleEnum<'a>),
    ConjunctionPlaceholder {
        module_name: &'a str,
        successors: Box<[&'a str]>,
        predecessors: Vec<&'a str>,
    },
}

impl<'a> ModuleOrPlaceholder<'a> {
    fn new_placeholder(module_name: &'a str, successors: Box<[&'a str]>) -> Self {
        ModuleOrPlaceholder::ConjunctionPlaceholder {
            module_name,
            successors,
            predecessors: Vec::new(),
        }
    }
}

fn parse_line<'a>(line: &'a str) -> ModuleOrPlaceholder<'a> {
    let (module_description, successors_str) = line.split_once(" -> ").unwrap();

    let successors = successors_str.split(", ").collect();

    let (first_char, module_name) = module_description.split_at(1);
    match first_char {
        "%" => ModuleOrPlaceholder::Module(FlipFlop::new(module_name, successors).into()),
        "&" => ModuleOrPlaceholder::new_placeholder(module_name, successors),
        "b" => {
            assert_eq!(
                module_description, "broadcaster",
                "Cannot parse \"{}\" into Module",
                line
            );
            ModuleOrPlaceholder::Module(Broadcaster::new(successors).into())
        }
        _ => panic!("Cannot parse \"{}\" into Module", line),
    }
}

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, ModuleEnum> {
    let temp_graph: HashMap<&str, RefCell<ModuleOrPlaceholder>> = input
        .lines()
        .map(parse_line)
        .map(|module_or_placeholder| {
            let module_name = match &module_or_placeholder {
                ModuleOrPlaceholder::Module(module) => module.name(),
                &ModuleOrPlaceholder::ConjunctionPlaceholder { module_name, .. } => module_name,
            };
            (module_name, RefCell::new(module_or_placeholder))
        })
        .collect();

    for (&module_name, module_or_placeholder_refcell) in temp_graph.iter() {
        let module_or_placeholder_ref = module_or_placeholder_refcell.borrow();
        let successors = match module_or_placeholder_ref.deref() {
            ModuleOrPlaceholder::Module(module) => module.successors(),
            ModuleOrPlaceholder::ConjunctionPlaceholder { successors, .. } => successors,
        };

        for &successor in successors {
            if let Some(other_module_or_placeholder_refcell) = temp_graph.get(successor) {
                let mut succ_module_ref = other_module_or_placeholder_refcell.borrow_mut();
                if let ModuleOrPlaceholder::ConjunctionPlaceholder { predecessors, .. } =
                    succ_module_ref.deref_mut()
                {
                    predecessors.push(module_name);
                }
            }
        }
    }

    // convert result into correct type by copying everything over
    let mut result: HashMap<&str, ModuleEnum> = temp_graph
        .into_iter()
        .map(|(module_name, module_or_placeholder_refcell)| {
            let module = match module_or_placeholder_refcell.into_inner() {
                ModuleOrPlaceholder::Module(module) => module,
                ModuleOrPlaceholder::ConjunctionPlaceholder {
                    successors,
                    predecessors,
                    ..
                } => Conjunction::new(module_name, successors, predecessors.into_boxed_slice())
                    .into(),
            };
            (module_name, module)
        })
        .collect();

    result.insert("button", Button.into());

    result
}

fn part1<'a>(mut graph: HashMap<&'a str, ModuleEnum<'a>>) -> usize {
    let mut num_low = 0;
    let mut num_high = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        Button.send_to_successors(Pulse::Low, &mut queue);

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

fn part2<'a>(mut graph: HashMap<&'a str, ModuleEnum<'a>>) -> usize {
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

    let jz_predecessors = if let ModuleEnum::Conjunction(conj) = &graph["jz"] {
        conj.predecessors().iter().cloned().collect::<Box<_>>()
    } else {
        panic!("jz is not a Conjunction Module");
    };

    let mut jz_predecessor_first_button_presses =
        vec![None; jz_predecessors.len()].into_boxed_slice();

    let mut num_jz_pred_found = 0;

    loop {
        num_button_presses += 1;

        let mut queue = VecDeque::new();
        Button.send_to_successors(Pulse::Low, &mut queue);

        while let Some(PulsePacket {
            source,
            destination,
            pulse,
        }) = queue.pop_front()
        {
            if matches!(pulse, Pulse::High) && destination == "jz" {
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
