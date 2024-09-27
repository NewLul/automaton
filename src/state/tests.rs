use crate::state::*;

#[test]
fn string_traversal_test() {
    assert_eq!(String::from("ab").go("abv").unwrap(), "v");
    assert_eq!(String::from("bd").go("bd").unwrap(), "");
    assert_eq!(String::from("").go("avcfd").unwrap(), "avcfd");
    assert_eq!(String::from("").go("").unwrap(), "");

    assert_eq!(String::from("ba").go("abv").is_none(), true);
    assert_eq!(String::from("afds").go("a").is_none(), true);
    assert_eq!(String::from("fds").go("avc").is_none(), true);
    assert_eq!(String::from("av").go("").is_none(), true);
}

#[test]
fn char_traversal_test() {
    assert_eq!('a'.go("abv").unwrap(), "bv");
    assert_eq!('b'.go("bd").unwrap(), "d");
    assert_eq!('c'.go("cavcfd").unwrap(), "avcfd");
    assert_eq!('d'.go("d").unwrap(), "");

    assert_eq!('e'.go("abv").is_none(), true);
    assert_eq!('f'.go("a").is_none(), true);
    assert_eq!('g'.go("avc").is_none(), true);
    assert_eq!('h'.go("").is_none(), true);
}