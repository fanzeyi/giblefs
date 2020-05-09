use crate::inode::Ino;
use fuse::{FileAttr, FileType};
use lazy_static::lazy_static;
use time::Timespec;

lazy_static! {
    static ref UNIX_EPOCH: Timespec = Timespec::new(0, 0);
}

pub trait ToFileAttr {
    fn to_file_attr(&self) -> FileAttr;
}

#[derive(Clone)]
pub struct FileAttrBuilder {
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    pub atime: Timespec,
    pub mtime: Timespec,
    pub ctime: Timespec,
    pub crtime: Timespec,
    pub kind: FileType,
    pub perm: u16,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
    pub flags: u32,
}

impl FileAttrBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn ino(mut self, ino: Ino) -> Self {
        self.ino = ino.0;
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size as u64;
        self
    }

    pub fn blocks(mut self, blocks: usize) -> Self {
        self.blocks = blocks as u64;
        self
    }

    pub fn time(mut self, time: Timespec) -> Self {
        self.atime = time;
        self.mtime = time;
        self.ctime = time;
        self.crtime = time;
        self
    }

    pub fn file(mut self) -> Self {
        self.kind = FileType::RegularFile;
        self.perm = 0o644;
        self
    }

    pub fn directory(mut self) -> Self {
        self.kind = FileType::Directory;
        self.perm = 0o755;
        self
    }

    pub fn nlink(mut self, nlink: u32) -> Self {
        self.nlink = nlink;
        self
    }

    pub fn build(self) -> FileAttr {
        FileAttr {
            ino: self.ino,
            size: self.size,
            blocks: self.blocks,
            atime: self.atime,
            mtime: self.mtime,
            ctime: self.ctime,
            crtime: self.crtime,
            kind: self.kind,
            perm: self.perm,
            nlink: self.nlink,
            uid: self.uid,
            gid: self.gid,
            rdev: self.rdev,
            flags: self.flags,
        }
    }
}

impl Default for FileAttrBuilder {
    fn default() -> Self {
        Self {
            ino: 0,
            size: 0,
            blocks: 0,
            atime: *UNIX_EPOCH,
            mtime: *UNIX_EPOCH,
            ctime: *UNIX_EPOCH,
            crtime: *UNIX_EPOCH,
            kind: FileType::RegularFile,
            perm: 0o644,
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        }
    }
}
