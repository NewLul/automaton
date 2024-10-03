use std::collections::{BTreeMap, BTreeSet, VecDeque};
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

    pub fn get_alphabet(&self) -> BTreeSet<char> {
        let mut alphabet: BTreeSet<char> = BTreeSet::new();
        for (_, state) in self.states.iter() {
            for (ch, _) in state.transitions.iter() {
                alphabet.insert(*ch);
            }
        }
        alphabet
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
        let mut needed = false;
        let alphabet = self.get_alphabet();
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

pub trait ToMinimalCompleteDfa {
    fn to_mcdfa(&self) -> Dfa;
}

impl ToMinimalCompleteDfa for Dfa {
    fn to_mcdfa(&self) -> Dfa {
        let cdfa = self.to_cdfa();
        let n: usize = *cdfa.states.keys().next_back().unwrap() as usize + 1;
        let mut reachable: Vec<bool> = vec![false; n];
        let mut edge: Vec<Vec<char>> = vec![vec!['\0'; n]; n];
        let mut is_terminal: Vec<bool> = vec![false; n];
        let mut marked: Vec<Vec<bool>> = vec![vec![false; n]; n];

        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(cdfa.starting_state as usize);
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            if reachable[cur] {
                continue;
            }
            reachable[cur] = true;
            is_terminal[cur] = cdfa.states[&(cur as i32)].is_terminal;
            for (ch, next_state) in cdfa.states[&(cur as i32)].transitions.clone() {
                edge[cur][next_state as usize] = ch;
                if !reachable[next_state as usize] {
                    queue.push_back(next_state as usize);
                }
            }
        }

        let mut pair_queue: VecDeque<(usize, usize)> = VecDeque::new();
        for i in 0..n {
            if !reachable[i] {
                continue;
            }
            for j in 0..n {
                if !reachable[j] {
                    continue;
                }
                if !marked[i][j] && is_terminal[i] != is_terminal[j] {
                    marked[i][j] = true;
                    marked[j][i] = true;
                    pair_queue.push_back((i, j));
                }
            }
        }

        while !pair_queue.is_empty() {
            let (u, v) = pair_queue.pop_front().unwrap();
            for i in 0..n {
                if !reachable[i] || edge[i][u] == '\0' {
                    continue;
                }
                for j in 0..n {
                    if !reachable[j] || edge[j][v] == '\0' {
                        continue;
                    }

                    if edge[i][u] == edge[j][v] && !marked[i][j] {
                        marked[i][j] = true;
                        marked[j][i] = true;
                        pair_queue.push_back((i, j));
                    }
                }
            }
        }

        
        let mut component: Vec<i32> = vec![-1; n];
        for i in 0..n {
            if !marked[cdfa.starting_state as usize][i] {
                component[i] = 0
            }
        }
        let mut component_count = 0;
        for i  in 0..n {
            if !reachable[i] {
                continue;
            }
            if component[i] == -1 {
                component_count += 1;
                component[i] = component_count;
                for j in (i + 1)..n {
                    if reachable[j] && !marked[i][j] {
                        component[j] = component_count;
                    }
                }
            }
        }

        let mut mcdfa = Dfa::new(component[cdfa.starting_state as usize]);
        
        for i in 0..(component_count + 1) {
            let mut is_terminal = true;
            let mut transitions: Vec<BTreeSet<char>> = vec![BTreeSet::new(); (component_count + 1) as usize];
            for j in 0..n {
                if component[j] != i || !reachable[j] {
                    continue;
                }
                is_terminal &= cdfa.states[&(j as i32)].is_terminal;
                for (ch, state) in cdfa.states[&(j as i32)].transitions.clone() {
                    transitions[component[state as usize] as usize].insert(ch);
                }
            }
            let mut new_state = DfaState::new(i, is_terminal);
            for (j, set) in transitions.iter().enumerate() {
                for ch in set {
                    new_state.add_transition(*ch, j as i32);
                }
            }
            mcdfa.add_state(new_state);
        }
        mcdfa
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