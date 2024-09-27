use crate::automaton::*;
use crate::nfa::*;

#[test]
fn nfa_base_test() {
    let mut q0: NfaState<String> = NfaState::new(0, true);
    q0.add_transition(String::from("ab"), 0);
    q0.add_transition(String::from(""), 1);
    
    let mut q1: NfaState<String> = NfaState::new(1, true);
    q1.add_transition(String::from("a"), 1);
    
    let mut nfa: Nfa<String> = Nfa::new(0);
    nfa.add_state(q0);
    nfa.add_state(q1);
    
    assert_eq!(nfa.accept(""), true);
    assert_eq!(nfa.accept("a"), true);
    assert_eq!(nfa.accept("ab"), true);
    assert_eq!(nfa.accept("aba"), true);
    assert_eq!(nfa.accept("c"), false);
    assert_eq!(nfa.accept("aab"), false);
    assert_eq!(nfa.accept("abababaa"), true);
    assert_eq!(nfa.accept("abababaab"), false);
}

#[test]
fn nfa_eps_cycle_test() {
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

    assert_eq!(nfa.accept(""), false);
    assert_eq!(nfa.accept("b"), true);
    assert_eq!(nfa.accept("bb"), false);
    assert_eq!(nfa.accept("aaaaaaaaaaaaab"), true);
    assert_eq!(nfa.accept("aaaaaaaaaaaabb"), false);
}