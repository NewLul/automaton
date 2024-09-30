use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt::Display;
use crate::automaton::Automaton;
use crate::state::*;
use std::fmt;

#[derive(Clone)]
pub struct NfaState<T: Traversable + Display + Clone> {
    pub index: i32,
    pub is_terminal: bool,
    transitions: Vec<(T, i32)>
}

impl<T: Traversable + Display + Clone> NfaState<T> {
    pub fn new(index: i32, is_terminal: bool) -> Self {
        Self{index: index, is_terminal: is_terminal, transitions: Vec::new()}
    }
}

impl<T: Traversable + Display + Clone> NfaState<T> {
    pub fn add_transition(&mut self, word: T, next_state: i32) {
        self.transitions.push((word, next_state));
    }
}

impl<T: Traversable + Display + Clone> State for NfaState<T> {
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

#[derive(Clone)]
pub struct Nfa<T: Traversable + Display + Clone> {
    starting_state: i32,
    states: BTreeMap<i32, NfaState<T>>
}

impl<T: Traversable + Display + Clone> Nfa<T> {
    pub fn new(starting_state: i32) -> Self {
        Self{starting_state: starting_state, states: BTreeMap::new()}
    }
    
    pub fn add_state(&mut self, state: NfaState<T>) {
        self.states.insert(state.index, state);
    }
}

impl<T: Traversable + Display + Clone> Automaton<'_> for Nfa<T> {
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

impl Nfa<String> {
    pub fn compress_eps(&self) -> Self {
        let mut result = self.clone();
        for (idx, _) in self.states.iter() {
            let mut new_transitions = vec![];
            let mut is_terminal = false;
            let mut used = vec![false; self.states.len()];
            let mut queue: VecDeque<i32> = VecDeque::new();
            queue.push_back(*idx);
            while !queue.is_empty() {
                let cur = queue.pop_front().unwrap();
                is_terminal |= result.states[&cur].is_terminal;
                used[cur as usize] = true;
                for (word, next_state) in self.states[&cur].transitions.iter() {
                    if word.len() > 0 {
                        new_transitions.push((word.clone(), *next_state));
                    } else if !used[*next_state as usize] {
                        queue.push_back(*next_state);
                    }
                }
            }
            result.states.get_mut(idx).unwrap().transitions = new_transitions;
            result.states.get_mut(idx).unwrap().is_terminal = is_terminal;
        }
        result
    }

    pub fn split_words(&self) -> Nfa<char> {
        let nfa = self.compress_eps();
        let mut result: Nfa<char> = Nfa::new(nfa.starting_state);

        for (idx, state) in nfa.states.iter() {
            result.add_state(NfaState::new(*idx, state.is_terminal));
        }

        for (idx, _) in nfa.states.iter() {
            for (word, next_state) in nfa.states[&idx].transitions.iter() {
                if word.len() == 1 {
                    result.states.get_mut(idx)
                                 .unwrap()
                                 .add_transition(word.chars().next().unwrap(), *next_state);
                } else {
                    for i in 0..word.len() - 1 {
                        let new_idx = *result.states.keys().next_back().unwrap() + 1;
                        result.states.get_mut(idx)
                                     .unwrap()
                                     .add_transition(word.chars().nth(i).unwrap(), new_idx);
                        result.add_state(NfaState::new(new_idx, false));
                        if i == word.len() - 2 {
                            result.states.get_mut(&new_idx)
                                         .unwrap()
                                         .add_transition(word.chars().nth(word.len() - 1).unwrap(), *next_state);
                        }
                    }
                }
            }
        }

        result
    }
}

impl<T: Traversable + Display + Clone> fmt::Display for Nfa<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Starting state: {}", self.starting_state)?;
        for (idx, state) in self.states.iter() {
            for (word, next_state) in state.transitions.iter() {
                writeln!(f, "{idx} -> {next_state} by \"{word}\"")?;
            }
        }
        writeln!(f, "Terminal states:")?;
        for (idx, state) in self.states.iter() {
            if state.is_terminal {
                writeln!(f, "{idx}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;