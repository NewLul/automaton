pub trait Automaton<'a> {
    fn accept(&self, str: &'a str) -> bool;
}

pub trait ToRegex {
    fn to_regex(&self) -> String;
}