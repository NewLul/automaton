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
fn test_mcdfa() {
    let mut a = DfaState::new(0, false);
    let mut b = DfaState::new(1, false);
    let mut c = DfaState::new(2, false);
    let mut d = DfaState::new(3, false);
    let mut e = DfaState::new(4, false);
    let mut f = DfaState::new(5, true);
    let mut g = DfaState::new(6, true);
    let mut h = DfaState::new(7, false);

    a.add_transition('0', 7);
    a.add_transition('1', 1);

    b.add_transition('0', 7);
    b.add_transition('1', 0);

    c.add_transition('0', 4);
    c.add_transition('1', 5);

    d.add_transition('0', 4);
    d.add_transition('1', 5);

    e.add_transition('0', 5);
    e.add_transition('1', 6);

    f.add_transition('0', 5);
    f.add_transition('1', 5);

    g.add_transition('0', 6);
    g.add_transition('1', 5);

    h.add_transition('0', 2);
    h.add_transition('1', 2);

    let mut dfa = Dfa::new(0);
    dfa.add_state(a);
    dfa.add_state(b);
    dfa.add_state(c);
    dfa.add_state(d);
    dfa.add_state(e);
    dfa.add_state(f);
    dfa.add_state(g);
    dfa.add_state(h);

    let mcdfa = dfa.to_mcdfa();

    assert_eq!(mcdfa.states.len(), 5);
    assert_eq!(mcdfa.accept("0000"), true);
    assert_eq!(mcdfa.accept("1110100"), true);
    assert_eq!(mcdfa.accept("100101"), true);
    assert_eq!(mcdfa.accept("1000010"), true);
    assert_eq!(mcdfa.accept("1000"), false);
    assert_eq!(mcdfa.accept("11101"), false);
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