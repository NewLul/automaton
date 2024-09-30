use std::collections::BTreeMap;
use std::fmt;

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

pub struct Dfa {
    pub starting_state: i32,
    pub states: BTreeMap<i32, DfaState>
}

impl Dfa {
    pub fn new(starting_state: i32) -> Self {
        Self{starting_state: starting_state, states: BTreeMap::new()}
    }
    
    pub fn add_state(&mut self, state: DfaState) {
        self.states.insert(state.index, state);
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