use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt::Display;
use crate::automaton::Automaton;
use crate::state::*;
use std::fmt;

pub struct NfaState<T: Traversable + Display> {
    pub index: i32,
    pub is_terminal: bool,
    transitions: Vec<(T, i32)>
}

impl<T: Traversable + Display> NfaState<T> {
    pub fn new(index: i32, is_terminal: bool) -> Self {
        Self{index: index, is_terminal: is_terminal, transitions: Vec::new()}
    }
}

impl<T: Traversable + Display> NfaState<T> {
    pub fn add_transition(&mut self, word: T, next_state: i32) {
        self.transitions.push((word, next_state));
    }
}

impl<T: Traversable + Display> State for NfaState<T> {
    fn next(&self, str: &str) -> Vec<(i32, String)> {
        let mut result = Vec::new();
        for (word, state) in &self.transitions {
            if let Some(value) = word.go(str) {
                result.push((state.clone(), 
                            value.to_string()));
            }
        }
        result
    }
}

pub struct Nfa<T: Traversable + Display> {
    starting_state: i32,
    states: BTreeMap<i32, NfaState<T>>
}

impl<T: Traversable + Display> Nfa<T> {
    pub fn new(starting_state: i32) -> Self {
        Self{starting_state: starting_state, states: BTreeMap::new()}
    }
    
    pub fn add_state(&mut self, state: NfaState<T>) {
        self.states.insert(state.index, state);
    }
}

impl<T: Traversable + Display> Automaton<'_> for Nfa<T> {
    fn accept<'a>(&self, str: &'a str) -> bool {
        let mut counter = (str.len() + 1) * (self.states.len() + 1) * (self.states.len() + 1);
        let mut queue: VecDeque<(i32, String)> = VecDeque::new();
        queue.push_back((self.starting_state, str.to_string()));
        while counter > 0 && !queue.is_empty() {
            counter -= 1;
            let (state, word) = queue.pop_front().unwrap();
            if word.len() == 0 && self.states[&state].is_terminal {
                return true
            }
            for (next_state, next_word) in self.states[&state].next(&word) {
                queue.push_back((next_state, next_word));
            }
        }
        return false
    }
}

impl<T: Traversable + Display> fmt::Display for Nfa<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Starting state: {}", self.starting_state)?;
        for (state, value) in self.states.iter() {
            for (word, next_state) in value.transitions.iter() {
                write!(f, "{state} -> {next_state} by \"{word}\"")?;
            }
        }
        writeln!(f, "Terminal states:")?;
        for (state, value) in self.states.iter() {
            if value.is_terminal {
                writeln!(f, "{state}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;