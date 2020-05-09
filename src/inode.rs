use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ino(pub u64);

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
        let ino = Ino(self.next_ino.fetch_add(1, Ordering::SeqCst));
        ino
    }
}

impl From<u64> for Ino {
    fn from(i: u64) -> Ino {
        Ino(i)
    }
}
