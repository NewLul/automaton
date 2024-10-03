use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::automaton::Automaton;

#[derive(Clone)]
pub struct DfaState {
    pub index: i32,
    pub is_terminal: bool,
    transitions: Vec<(char, i32)>
}

impl DfaState {
    pub fn new(index: i32, is_terminal: bool) -> Self {
        Self{index: index, is_terminal: is_terminal, transitions: Vec::new()}
    }

    pub fn add_transition(&mut self, sym: char, next_state: i32) {
        self.transitions.push((sym, next_state));
    }
}

#[derive(Clone)]
pub struct Dfa {
    pub starting_state: i32,
    pub states: BTreeMap<i32, DfaState>
}

impl Dfa {
    pub fn new(starting_state: i32) -> Self {
        Self{starting_state: starting_state, states: BTreeMap::new()}
    }
    
    pub fn add_state(&mut self, state: DfaState) {
        if self.states.contains_key(&state.index) {
            panic!("State index duplicate!");
        }
        self.states.insert(state.index, state);
    }
}

impl Automaton<'_> for Dfa {
    fn accept<'a>(&self, str: &'a str) -> bool {
        let mut cur: i32 = self.starting_state;
        for i in 0..str.len() {
            let mut success = false;
            for (ch, next_state) in self.states[&cur].transitions.iter() {
                if *ch == str.chars().nth(i).unwrap() {
                    cur = *next_state;
                    success = true;
                    break;
                }
            }
            if !success {
                return false
            }
        }
        self.states[&cur].is_terminal
    }
}

pub trait ToCompleteDfa {
    fn to_cdfa(&self) -> Dfa;
}

impl ToCompleteDfa for Dfa {
    fn to_cdfa(&self) -> Dfa {
        let mut cdfa = self.clone();
        let mut alphabet: BTreeSet<char> = BTreeSet::new();
        let mut needed = false;
        for (_, state) in self.states.iter() {
            for (ch, _) in state.transitions.iter() {
                alphabet.insert(*ch);
            }
        }
        let virtual_index = self.states.keys().next_back().unwrap() + 1;
        let mut virtual_state = DfaState::new(virtual_index, false);
        for ch in &alphabet {
            virtual_state.add_transition(*ch, virtual_index);
        }
        for (_, state) in cdfa.states.iter_mut() {
            let mut exist: BTreeSet<char> = BTreeSet::new();
            for (ch, _) in state.transitions.iter() {
                exist.insert(*ch);
            }
            for ch in &alphabet {
                if !exist.contains(ch) {
                    needed = true;
                    state.add_transition(*ch, virtual_index);
                }
            }
        }
        if needed {
            cdfa.add_state(virtual_state);
        }
        cdfa
    }
}
pub trait ToDfa {
    fn to_dfa(&self) -> Dfa;
}

impl fmt::Display for Dfa {
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