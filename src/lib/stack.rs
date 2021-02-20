#[derive(Copy, Clone)]
pub enum Element {
    Data(u128),
    Frame(u128),
    Return(u128),
    Register(u128),
}

impl Element {
    pub fn is_data(self) -> bool {
        match self {
            Self::Data(_) => true,
            _ => false,
        }
    }

    pub fn is_control(self) -> bool {
        match self {
            Self::Data(_) => false,
            _ => true,
        }
    }

    pub fn get_frame(self) -> Option<Element> {
        match self {
            Self::Frame(_) => Some(self),
            _ => None,
        }
    }

    pub fn get_return(self) -> Option<Element> {
        match self {
            Self::Return(_) => Some(self),
            _ => None,
        }
    }

    pub fn get_register(self) -> Option<Element> {
        match self {
            Self::Register(_) => Some(self),
            _ => None,
        }
    }

    pub fn get_data(self) -> Option<u128> {
        match self {
            Self::Data(x) => Some(x),
            _ => None,
        }
    }
}

pub struct Stack {
    data: Vec<Option<Element>>,
    sp: Option<usize>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            sp: None,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, e: Element) -> Option<Element> {
        None
    }

    pub fn pop(&mut self) -> Option<Element> {
        None
    }

    pub fn store_registers(&mut self, _rs: &[u128]) {}

    pub fn recall_registers(&mut self, _rs: &mut [u128]) {}

    /// Appends a new frame with the given control elements then copies the top n elements as data arguments.
    pub fn append_frame(&mut self, control_elems: &[Element], copy_args: usize) {
        let sp = self.len();
        let n_control = control_elems.len();
        self.data.resize(self.len() + n_control, None); // Resize the stack to fit new control_elems.
        let (src, dst) = self.data.split_at_mut(sp - copy_args); // Split the stack into two mutable parts.
        dst[n_control..].copy_from_slice(&src[src.len() - copy_args..]); // Copy arguments over.
    }

    /// Truncates the current frame and copy the top n elements as return data values.
    pub fn truncate_frame(&mut self, copy_returns: usize) -> Result<(), ()> {
        Err(())
    }
}
