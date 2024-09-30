mod automaton;
mod state;
mod nfa;

use nfa::*;

fn main() {
    let mut q0: NfaState<String> = NfaState::new(0, false);
    q0.add_transition(String::from(""), 1);
    
    let mut q1: NfaState<String> = NfaState::new(1, false);
    q1.add_transition(String::from(""), 2);
    q1.add_transition(String::from("b"), 3);

    let mut q2: NfaState<String> = NfaState::new(2, false);
    q2.add_transition(String::from("a"), 0);

    let q3: NfaState<String> = NfaState::new(3, true);

    let mut nfa: Nfa<String> = Nfa::new(0);
    nfa.add_state(q0);
    nfa.add_state(q1);
    nfa.add_state(q2);
    nfa.add_state(q3);

    print!("{}", nfa);
    
    nfa = nfa.compress_eps();
    
    print!("{}", nfa);
}