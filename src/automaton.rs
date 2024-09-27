pub trait Automaton<'a> {
    fn accept(&self, str: &'a str) -> bool;
}
