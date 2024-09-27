use std::collections::HashMap;
use std::collections::VecDeque;
use crate::automaton::Automaton;
use crate::state::*;

pub struct NfaState<T: Traversable + Clone> {
    pub name: String,
    pub is_terminal: bool,
    transitions: Vec<(T, String)>
}

impl<T: Traversable + Clone> NfaState<T> {
    pub fn new(name: String, is_terminal: bool) -> Self {
        Self{name: name, is_terminal: is_terminal, transitions: Vec::new()}
    }
}

impl<T: Traversable + Clone> NfaState<T> {
    pub fn add_transition(&mut self, word: T, next_state: String) {
        self.transitions.push((word, next_state));
    }
}

impl<T: Traversable + Clone> State for NfaState<T> {
    fn next(&self, str: &str) -> Vec<(String, String)> {
        let mut result = Vec::new();
        for (word, state) in &self.transitions {
            if let Some(value) = word.go(str) {
                result.push((state.to_string(), 
                            value.to_string()));
            }
        }
        result
    }
}

pub struct Nfa<T: Traversable + Clone> {
    starting_state: String,
    states: HashMap<String, NfaState<T>>
}

impl<T: Traversable + Clone> Nfa<T> {
    pub fn new(starting_state: String) -> Self {
        Self{starting_state: starting_state, states: HashMap::new()}
    }
    
    pub fn add_state(&mut self, state: NfaState<T>) {
        self.states.insert(state.name.clone(), state);
    }
}

impl<T: Traversable + Clone> Automaton<'_> for Nfa<T> {
    fn accept<'a>(&self, str: &'a str) -> bool {
        let mut counter = (str.len() + 1) * (self.states.len() + 1) * (self.states.len() + 1);
        let mut queue: VecDeque<(String, String)> = VecDeque::new();
        queue.push_back((self.starting_state.clone(), str.to_string()));
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

#[cfg(test)]
mod tests;