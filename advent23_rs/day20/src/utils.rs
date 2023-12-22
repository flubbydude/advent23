use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub enum Pulse {
    Low,
    High,
}

pub struct PulsePacket<'a> {
    pub source: &'a str,
    pub destination: &'a str,
    pub pulse: Pulse,
}

#[enum_dispatch(ModuleEnum)]
pub trait Module<'a> {
    fn name(&self) -> &'a str;
    fn successors(&self) -> &[&'a str];
    fn process_pulse(
        &mut self,
        source: &'a str,
        pulse: Pulse,
        queue: &mut VecDeque<PulsePacket<'a>>,
    );
    fn send_to_successors(&self, pulse: Pulse, queue: &mut VecDeque<PulsePacket<'a>>) {
        for successor in self.successors() {
            queue.push_back(PulsePacket {
                source: self.name(),
                destination: successor,
                pulse,
            })
        }
    }
}

#[derive(Clone)]
pub struct FlipFlop<'a> {
    name: &'a str,
    successors: Box<[&'a str]>,
    on: bool,
}

impl<'a> FlipFlop<'a> {
    pub fn new(name: &'a str, successors: Box<[&'a str]>) -> Self {
        FlipFlop {
            name,
            successors,
            on: false,
        }
    }
}

impl<'a> Module<'a> for FlipFlop<'a> {
    fn name(&self) -> &'a str {
        self.name
    }

    fn successors(&self) -> &[&'a str] {
        &self.successors
    }

    fn process_pulse(
        &mut self,
        _source: &str,
        pulse: Pulse,
        queue: &mut VecDeque<PulsePacket<'a>>,
    ) {
        if matches!(pulse, Pulse::High) {
            return;
        }
        self.on = !self.on;

        let pulse_to_send = if self.on { Pulse::High } else { Pulse::Low };

        self.send_to_successors(pulse_to_send, queue);
    }
}

#[derive(Clone)]
pub struct Conjunction<'a> {
    // most_recent_pulses: HashMap<&'a str, Pulse>,
    name: &'a str,
    successors: Box<[&'a str]>,
    predecessors: Box<[&'a str]>,
    most_recent_pulses: Box<[Pulse]>,
}

impl<'a> Conjunction<'a> {
    pub fn new(name: &'a str, successors: Box<[&'a str]>, predecessors: Box<[&'a str]>) -> Self {
        Conjunction {
            name,
            successors,
            most_recent_pulses: vec![Pulse::Low; predecessors.len()].into_boxed_slice(),
            predecessors,
        }
    }

    pub fn predecessors(&self) -> &[&'a str] {
        &self.predecessors
    }
}

impl<'a> Module<'a> for Conjunction<'a> {
    fn name(&self) -> &'a str {
        self.name
    }

    fn successors(&self) -> &[&'a str] {
        &self.successors
    }

    fn process_pulse(&mut self, source: &str, pulse: Pulse, queue: &mut VecDeque<PulsePacket<'a>>) {
        self.most_recent_pulses[self
            .predecessors
            .iter()
            .position(|&pred| pred == source)
            .unwrap()] = pulse;

        let pulse_to_send = if self
            .most_recent_pulses
            .iter()
            .all(|pulse| matches!(pulse, Pulse::High))
        {
            Pulse::Low
        } else {
            Pulse::High
        };

        self.send_to_successors(pulse_to_send, queue);
    }
}

#[derive(Clone)]
pub struct Broadcaster<'a> {
    successors: Box<[&'a str]>,
}

impl<'a> Broadcaster<'a> {
    pub fn new(successors: Box<[&'a str]>) -> Self {
        Broadcaster { successors }
    }
}

impl<'a> Module<'a> for Broadcaster<'a> {
    fn name(&self) -> &'static str {
        "broadcaster"
    }

    fn successors(&self) -> &[&'a str] {
        &self.successors
    }

    fn process_pulse(
        &mut self,
        _source: &str,
        pulse: Pulse,
        queue: &mut VecDeque<PulsePacket<'a>>,
    ) {
        self.send_to_successors(pulse, queue);
    }
}

#[derive(Clone)]
pub struct Button;

impl Module<'_> for Button {
    fn name(&self) -> &'static str {
        "broadcaster"
    }

    fn successors(&self) -> &[&'static str] {
        lazy_static! {
            static ref BUTTON_SUCCESSORS: Box<[&'static str]> = Box::new(["broadcaster"]);
        };

        &BUTTON_SUCCESSORS
    }

    fn process_pulse(&mut self, _source: &str, _pulse: Pulse, _queue: &mut VecDeque<PulsePacket>) {
        panic!("Button cannot receive a pulse")
    }
}

#[enum_dispatch]
#[derive(Clone)]
pub enum ModuleEnum<'a> {
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>),
    Broadcaster(Broadcaster<'a>),
    Button,
}
