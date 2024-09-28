pub trait Traversable {
    fn go<'a>(&self, str: &'a str) -> Option<&'a str>;
}

impl Traversable for String {
    fn go<'a>(&self, str: &'a str) -> Option<&'a str> {
        str.strip_prefix(self)
    }
}

impl Traversable for char {
    fn go<'a>(&self, str: &'a str) -> Option<&'a str> {
        str.strip_prefix(*self)
    }
}

pub trait State { 
    fn next(&self, str: &str) -> Vec<(i32, String)>;
}

#[cfg(test)]
mod tests;