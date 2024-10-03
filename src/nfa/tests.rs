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

#[test]
fn nfa_to_dfa_test() {
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

    let dfa = nfa.to_dfa();

    assert_eq!(dfa.accept("a"), true);
    assert_eq!(dfa.accept("bb"), false);
    assert_eq!(dfa.accept("ba"), true);
    assert_eq!(dfa.accept("bba"), true);
    assert_eq!(dfa.accept("abba"), true);
}

#[test]
fn nfa_to_dfa_size_test() {
    let mut q0: NfaState<char> = NfaState::new(0, false);
    q0.add_transition('a', 0);
    q0.add_transition('b', 0);
    q0.add_transition('b', 1);
    
    let mut q1: NfaState<char> = NfaState::new(1, false);
    q1.add_transition('a', 2);
    q1.add_transition('b', 2);
    
    let mut q2: NfaState<char> = NfaState::new(2, false);
    q2.add_transition('a', 3);
    q2.add_transition('b', 3);

    let mut q3: NfaState<char> = NfaState::new(3, false);
    q3.add_transition('a', 4);
    q3.add_transition('b', 4);

    let q4: NfaState<char> = NfaState::new(4, true);

    let mut nfa: Nfa<char> = Nfa::new(0);
    nfa.add_state(q0);
    nfa.add_state(q1);
    nfa.add_state(q2);
    nfa.add_state(q3);
    nfa.add_state(q4);

    assert_eq!(nfa.to_dfa().states.len(), 16);
}

#[test]
fn nfa_to_mcdfa() {
    let mut q0: NfaState<char> = NfaState::new(0, false);
    q0.add_transition('a', 1);
    q0.add_transition('a', 2);
    q0.add_transition('b', 2);
    
    let mut q1: NfaState<char> = NfaState::new(1, false);
    q1.add_transition('a', 2);
    q1.add_transition('b', 3);
    
    let mut q2: NfaState<char> = NfaState::new(2, false);
    q2.add_transition('a', 1);
    q2.add_transition('a', 2);
    q2.add_transition('b', 3);

    let q3: NfaState<char> = NfaState::new(3, true);

    let mut nfa: Nfa<char> = Nfa::new(0);
    nfa.add_state(q0);
    nfa.add_state(q1);
    nfa.add_state(q2);
    nfa.add_state(q3);

    let mcdfa = nfa.to_mcdfa();

    assert_eq!(mcdfa.states.len(), 4);
    assert_eq!(mcdfa.accept("ab"), true);
    assert_eq!(mcdfa.accept("aaaaaab"), true);
    assert_eq!(mcdfa.accept("baab"), true);
    assert_eq!(mcdfa.accept("bb"), true);
    assert_eq!(mcdfa.accept("aaaa"), false);
    assert_eq!(mcdfa.accept("bbb"), false);
}


#[test]
#[should_panic]
fn duplicate_index_test() {
    let q0: NfaState<char> = NfaState::new(0, false);
    let q1: NfaState<char> = NfaState::new(0, false);
    let mut nfa: Nfa<char> = Nfa::new(0);
    nfa.add_state(q0);
    nfa.add_state(q1);
}