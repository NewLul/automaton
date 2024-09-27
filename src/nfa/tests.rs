use crate::automaton::*;
use crate::nfa::*;

#[test]
fn nfa_base_test() {
    let mut q0: NfaState<String> = NfaState::new(String::from("q0"), true);
    q0.add_transition(String::from("ab"), String::from("q0"));
    q0.add_transition(String::from(""), String::from("q1"));
    
    let mut q1: NfaState<String> = NfaState::new(String::from("q1"), true);
    q1.add_transition(String::from("a"), String::from("q1"));
    
    let mut nfa: Nfa<String> = Nfa::new(String::from("q0"));
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
    let mut q0: NfaState<String> = NfaState::new(String::from("q0"), false);
    q0.add_transition(String::from(""), String::from("q1"));
    
    let mut q1: NfaState<String> = NfaState::new(String::from("q1"), false);
    q1.add_transition(String::from(""), String::from("q2"));
    q1.add_transition(String::from("b"), String::from("q3"));

    let mut q2: NfaState<String> = NfaState::new(String::from("q2"), false);
    q2.add_transition(String::from("a"), String::from("q0"));

    let q3: NfaState<String> = NfaState::new(String::from("q3"), true);

    let mut nfa: Nfa<String> = Nfa::new(String::from("q0"));
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