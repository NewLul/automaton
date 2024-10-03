use crate::dfa::*;

#[test]
fn dfa_to_cdfa_test() {
    let mut q0 = DfaState::new(0, false);
    q0.add_transition('a', 0);
    q0.add_transition('b', 1);
    
    let mut q1 = DfaState::new(1, true);
    q1.add_transition('b', 1);
    
    let mut dfa = Dfa::new(0);
    dfa.add_state(q0);
    dfa.add_state(q1);
    
    assert_eq!(dfa.to_cdfa().states.len(), 3);
}

#[test]
#[should_panic]
fn duplicate_index_test() {
    let q0 = DfaState::new(0, false);
    let q1 = DfaState::new(0, false);
    let mut dfa = Dfa::new(0);
    dfa.add_state(q0);
    dfa.add_state(q1);
}