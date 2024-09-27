pub trait Traversable {
    fn go<'a>(&'a self, str: &'a str) -> Option<&str>;
}

impl Traversable for String {
    fn go<'a>(&'a self, str: &'a str) -> Option<&str> {
        str.strip_prefix(self)
    }
}

impl Traversable for char {
    fn go<'a>(&'a self, str: &'a str) -> Option<&str> {
        str.strip_prefix(*self)
    }
}

pub trait State { 
    fn next(&self, str: &str) -> Vec<(String, String)>;
}

#[cfg(test)]
mod tests;