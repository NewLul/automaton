mod automaton;
mod state;
mod nfa;
mod dfa;

use automaton::ToRegex;

use crate::nfa::*;
fn main() {
    let mut q0: NfaState<String> = NfaState::new(0, false);
    q0.add_transition(String::from("a"), 0);
    q0.add_transition(String::from("b"), 0);
    q0.add_transition(String::from("a"), 1);
    
    let mut q1: NfaState<String> = NfaState::new(1, true);
    q1.add_transition(String::from("b"), 1);
    q1.add_transition(String::from("b"), 0);
    
    let mut nfa: Nfa<String> = Nfa::new(0);
    nfa.add_state(q0);
    nfa.add_state(q1);

    println!("{}", nfa.to_regex());
}