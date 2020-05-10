use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ino {
    no: u64,
    parent: u64,
}

impl Ino {
    pub fn new(no: u64) -> Ino {
        Ino { no, parent: 1 }
    }

    pub fn set_parent(&mut self, parent: Ino) {
        self.parent = parent.no;
    }

    pub fn value(&self) -> u64 {
        self.no
    }

    pub fn parent(&self) -> u64 {
        self.parent
    }
}

impl From<u64> for Ino {
    fn from(i: u64) -> Ino {
        Ino::new(i)
    }
}

pub trait Inode {
    fn ino(&self) -> Ino;
}

pub struct InodeGen {
    next_ino: AtomicU64,
}

impl InodeGen {
    pub fn new() -> Self {
        InodeGen {
            next_ino: AtomicU64::new(2),
        }
    }

    pub fn next(&self) -> Ino {
        let ino = Ino::new(self.next_ino.fetch_add(1, Ordering::SeqCst));
        ino
    }
}
