use std::collections::{BTreeMap, VecDeque, BTreeSet};
use std::fmt::Display;
use crate::automaton::*;
use crate::state::*;
use crate::dfa::{Dfa, DfaState, ToCompleteDfa, ToMinimalCompleteDfa, ToDfa};
use std::fmt;

#[derive(Clone)]
pub struct NfaState<T: Traversable + Display + Clone> {
    pub index: usize,
    pub is_terminal: bool,
    transitions: Vec<(T, usize)>
}

impl<T: Traversable + Display + Clone> NfaState<T> {
    pub fn new(index: usize, is_terminal: bool) -> Self {
        Self{index: index, is_terminal: is_terminal, transitions: Vec::new()}
    }
}

impl<T: Traversable + Display + Clone> NfaState<T> {
    pub fn add_transition(&mut self, word: T, next_state: usize) {
        self.transitions.push((word, next_state));
    }
}

impl<T: Traversable + Display + Clone> State for NfaState<T> {
    fn next(&self, str: &str) -> Vec<(usize, String)> {
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
    pub starting_state: usize,
    pub states: BTreeMap<usize, NfaState<T>>
}

impl<T: Traversable + Display + Clone> Nfa<T> {
    pub fn new(starting_state: usize) -> Self {
        Self{starting_state: starting_state, states: BTreeMap::new()}
    }
    
    pub fn add_state(&mut self, state: NfaState<T>) {
        if self.states.contains_key(&state.index) {
            panic!("State index duplicate!");
        }
        self.states.insert(state.index, state);
    }
}

impl<T: Traversable + Display + Clone> Automaton<'_> for Nfa<T> {
    fn accept<'a>(&self, str: &'a str) -> bool {
        let mut counter = (str.len() + 1) * (self.states.len() + 1) * (self.states.len() + 1);
        let mut queue: VecDeque<(usize, String)> = VecDeque::new();
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
            let n: usize = *self.states.keys().next_back().unwrap() as usize + 1;
            let mut used = vec![false; n];
            let mut queue: VecDeque<usize> = VecDeque::new();
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

impl ToDfa for Nfa<char> {
    fn to_dfa(&self) -> Dfa {
        let mut dfa = Dfa::new(1 << self.starting_state);
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut alphabet: BTreeSet<char> = BTreeSet::new();
        let n: usize = *self.states.keys().next_back().unwrap() as usize + 1;
        for (_, state) in self.states.iter() {
            for (ch, _) in state.transitions.iter() {
                alphabet.insert(*ch);
            }
        }
        queue.push_back(1 << self.starting_state);
        while !queue.is_empty() {
            let cur = queue.pop_back().unwrap();
            if dfa.states.contains_key(&cur) {
                continue;
            }
            let mut is_terminal = false;
            for i in 0..n {
                if (cur >> i & 1) == 0 {
                    continue;
                }
                is_terminal |= self.states[&i].is_terminal;
            }
            let mut cur_state = DfaState::new(cur, is_terminal);
            for ch in alphabet.iter() {
                let mut q = 0;
                for i in 0..n {
                    if (cur >> i & 1) == 0 {
                        continue;
                    }
                    for (transition, idx) in self.states[&i].transitions.iter() {
                        if transition == ch {
                            q |= 1 << idx;
                        }
                    }
                }
                if q == 0 {
                    continue;
                }
                cur_state.add_transition(*ch, q);
                if !dfa.states.contains_key(&q) {
                    queue.push_back(q);
                }
            }
            dfa.add_state(cur_state);
        }
        dfa
    }
}

impl ToDfa for Nfa<String> {
    fn to_dfa(&self) -> Dfa {
        let a = self.compress_eps();
        let b = a.split_words();
        b.to_dfa()
    }
}

impl ToCompleteDfa for Nfa<String> {
    fn to_cdfa(&self) -> Dfa {
        self.to_dfa().to_cdfa()
    }
}

impl ToCompleteDfa for Nfa<char> {
    fn to_cdfa(&self) -> Dfa {
        self.to_dfa().to_cdfa()
    }
}

impl ToMinimalCompleteDfa for Nfa<String> {
    fn to_mcdfa(&self) -> Dfa {
        self.to_cdfa().to_mcdfa()
    }
}

impl ToMinimalCompleteDfa for Nfa<char> {
    fn to_mcdfa(&self) -> Dfa {
        self.to_cdfa().to_mcdfa()
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

impl ToRegex for Nfa<String> {
    fn to_regex(&self) -> String {
        let mut new_nfa = self.to_dfa().to_nfa();
        let max = new_nfa.states.keys().next_back().unwrap().clone();
        
        let mut new_start: NfaState<String> = NfaState::new(max + 1, false);
        new_start.add_transition(String::from(""), new_nfa.starting_state);
        new_nfa.add_state(new_start);
        new_nfa.starting_state = max + 1;

        let new_end: NfaState<String> = NfaState::new(max + 2, true);
        for (_, state) in new_nfa.states.iter_mut() {
            if state.is_terminal {
                state.is_terminal = false;
                state.add_transition(String::from(""), max + 2);
            }
        }
        new_nfa.add_state(new_end);

        for i in 0..(max + 1) {
            if !new_nfa.states.contains_key(&i) {
                continue;
            }
            let mut new_transitions: BTreeMap<usize, (String, bool)> = BTreeMap::new();
            for (word, idx) in new_nfa.states[&i].transitions.clone() {
                if new_transitions.contains_key(&idx) {
                    let it = new_transitions.get_mut(&idx).unwrap();
                    it.0.push_str("+");
                    it.0.push_str(&word);
                    it.1 = true;
                } else {
                    new_transitions.insert(idx, (word, false));
                }
            }
            let mut update: Vec<(String, usize)> = vec![];
            for (idx, (word, more_than_one)) in new_transitions.iter_mut() {
                if *more_than_one {
                    *word = "(".to_owned() + word + ")";
                }
                update.push((word.to_string(), *idx));
            }
            new_nfa.states.get_mut(&i).unwrap().transitions = update; 
        }

        for i in 0..(max + 1) {
            if !new_nfa.states.contains_key(&i) {
                continue;
            }
            let mut self_loop: String = String::from("");
            let mut self_loop_idx = -1;
            for (k, (word, idx)) in new_nfa.states[&i].transitions.iter().enumerate() {
                if *idx == i && word.len() > 0 {
                    self_loop = String::from("(");
                    self_loop.push_str(word);
                    self_loop.push_str(")*");
                    self_loop_idx = k as i32;
                    break;
                }
            }
            if self_loop_idx != -1 {
                new_nfa.states.get_mut(&i).unwrap().transitions.remove(self_loop_idx as usize);
            }
            for j in 0..(max + 3) {
                if !new_nfa.states.contains_key(&j) {
                    continue;
                }
                let mut new_transitions: BTreeMap<usize, Vec<String>> = BTreeMap::new(); 
                let mut delete: i32 = -1;
                for (k, (word, idx)) in new_nfa.states[&j].transitions.iter().enumerate() {
                    if *idx == i {
                        for (second_word, final_state) in new_nfa.states[&i].transitions.iter() {
                            let mut transition = word.clone();
                            transition.push_str(&self_loop);
                            transition.push_str(second_word);
                            if new_transitions.contains_key(final_state) {
                                new_transitions.get_mut(final_state).unwrap().push(transition);
                            } else {
                                new_transitions.insert(*final_state, vec![transition]);
                            }
                        }
                        delete = k as i32;
                        break;
                    }
                }
                if delete == -1 {
                    continue
                }
                new_nfa.states.get_mut(&j).unwrap().transitions.remove(delete as usize);
                for (word, idx) in new_nfa.states.get_mut(&j).unwrap().transitions.iter_mut() {
                    if new_transitions.contains_key(idx) {
                        if new_transitions.len() > 1 {
                            *word = "(".to_owned() + word;
                        }
                        for transition in new_transitions.get(idx).unwrap() {
                            word.push_str("+");
                            word.push_str(&transition);
                        }
                        if new_transitions.len() > 1 {
                            word.push_str(")");
                        }
                        new_transitions.remove(idx);
                    }
                }
                for (next_state, words) in new_transitions.iter() {
                    new_nfa.states.get_mut(&j).unwrap().add_transition(words.join(""), *next_state);
                }
            }
            new_nfa.states.remove(&i);
        }

        new_nfa.states.get(&(max + 1)).unwrap().transitions.get(0).unwrap().0.clone()
    }
}

impl Nfa<char> {
    fn to_nfa_string(&self) -> Nfa<String> {
        let mut nfa: Nfa<String> = Nfa::new(self.starting_state);
        for (idx, state) in self.states.iter() {
            let mut nfa_state: NfaState<String> = NfaState::new(*idx, state.is_terminal);
            for (ch, next_state) in state.transitions.iter() {
                nfa_state.add_transition(String::from(*ch), *next_state);
            }
            nfa.add_state(nfa_state);
        }
        nfa
    }
}

impl ToRegex for Nfa<char> {
    fn to_regex(&self) -> String {
        self.to_nfa_string().to_regex()
    }
}

#[cfg(test)]
mod tests;