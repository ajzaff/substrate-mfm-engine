use std::fmt;

pub const RandomRegister: Register = Register(15);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Register(pub usize);

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct SiteNumber(pub u8);

impl fmt::Display for SiteNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub sel: FieldSelector,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.sel)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FieldSelector {
    pub offset: u8,
    pub length: u8,
}

impl fmt::Display for FieldSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}:{}}}", self.offset, self.length)
    }
}

#[derive(Clone, Debug)]
pub struct Parameter<T> {
    pub name: String,
    pub value: T,
}
