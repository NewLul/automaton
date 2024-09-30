mod automaton;
mod state;
mod nfa;
mod dfa;

use nfa::*;
use dfa::*;

fn main() {
    let mut q0: NfaState<char> = NfaState::new(0, false);
    q0.add_transition('a', 0);
    q0.add_transition('b', 0);
    q0.add_transition('a', 1);
    
    let mut q1: NfaState<char> = NfaState::new(1, true);
    q1.add_transition('b', 1);
    q1.add_transition('b', 0);
    
    let mut nfa: Nfa<char> = Nfa::new(0);
    nfa.add_state(q0);
    nfa.add_state(q1);

    print!("{}", nfa);
    
    let dfa = nfa.to_dfa();
    
    print!("{}", dfa);
}