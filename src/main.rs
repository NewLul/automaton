mod automaton;
mod state;
mod nfa;

use nfa::*;

fn main() {
    let mut q0: NfaState<String> = NfaState::new(0, true);
    q0.add_transition(String::from("ab"), 0);
    q0.add_transition(String::from(""), 1);
    
    let mut q1: NfaState<String> = NfaState::new(1, true);
    q1.add_transition(String::from("a"), 1);
    
    let mut nfa: Nfa<String> = Nfa::new(0);
    nfa.add_state(q0);
    nfa.add_state(q1);

    print!("{}", nfa);
    
    let nfaa = nfa.split_words();
    
    print!("{}", nfaa);
}