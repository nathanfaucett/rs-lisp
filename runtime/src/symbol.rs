use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(String);

impl fmt::Debug for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Into<String> for Symbol {
    #[inline]
    fn into(self) -> String {
        self.0
    }
}

impl Symbol {
    #[inline]
    pub fn new(value: String) -> Self {
        Symbol(value)
    }

    #[inline]
    pub fn inner(&self) -> &String {
        &self.0
    }
    #[inline]
    pub fn inner_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
