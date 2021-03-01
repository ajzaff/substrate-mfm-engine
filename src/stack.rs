use std::ops::IndexMut;
use std::vec::Vec;

enum ControlEntry<T, U> {
    Frame(usize),
    Return(usize),
    Register(T),
    User(U),
}

enum Entry<T, U> {
    Control(ControlEntry<T, U>),
    Data(T),
}

pub struct Stack<T, U> {
    data: Vec<Entry<T, U>>,
}

impl<T, U> Stack<T, U> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push_entry(self: &mut Self, e: Entry<T, U>) {
        self.data.push(e)
    }

    pub fn push(self: &mut Self, e: T) {
        self.push_entry(Entry::Data(e))
    }

    pub fn push_control(self: &mut Self, e: U) {
        self.push_entry(Entry::Control(ControlEntry::User(e)))
    }
}
