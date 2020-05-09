use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use lazy_static::lazy_static;
use libc::ENOENT;
use log::debug;
use std::ffi::OsStr;
use time::Timespec;

const HELLO_TXT_CONTENT: &str = "Hello World!\n";

lazy_static! {
    static ref UNIX_EPOCH: Timespec = Timespec::new(0, 0);
    static ref TTL: Timespec = Timespec::new(1, 0);
    static ref HELLO_DIR_ATTR: FileAttr = FileAttr {
        ino: 1,
        size: 0,
        blocks: 0,
        atime: *UNIX_EPOCH, // 1970-01-01 00:00:00
        mtime: *UNIX_EPOCH,
        ctime: *UNIX_EPOCH,
        crtime: *UNIX_EPOCH,
        kind: FileType::Directory,
        perm: 0o755,
        nlink: 2,
        uid: 501,
        gid: 20,
        rdev: 0,
        flags: 0,
    };
    static ref HELLO_TXT_ATTR: FileAttr = FileAttr {
        ino: 2,
        size: 13,
        blocks: 1,
        atime: *UNIX_EPOCH, // 1970-01-01 00:00:00
        mtime: *UNIX_EPOCH,
        ctime: *UNIX_EPOCH,
        crtime: *UNIX_EPOCH,
        kind: FileType::RegularFile,
        perm: 0o644,
        nlink: 1,
        uid: 501,
        gid: 20,
        rdev: 0,
        flags: 0,
    };
}

pub struct GilberFS {}

impl GilberFS {
    fn log_request(&self, req: &Request) {
        debug!("processing request: {:?}", req);
    }
}

impl Filesystem for GilberFS {
    fn lookup(&mut self, req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        self.log_request(req);

        if parent == 1 && name.to_str() == Some("hello.txt") {
            reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, req: &Request, ino: u64, reply: ReplyAttr) {
        self.log_request(req);

        match ino {
            1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
            2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
            _ => reply.error(ENOENT),
        }
    }

    fn read(
        &mut self,
        req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        reply: ReplyData,
    ) {
        self.log_request(req);

        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(
        &mut self,
        req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        self.log_request(req);

        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
            (2, FileType::RegularFile, "hello.txt"),
        ];

        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            // i + 1 means the index of the next entry
            reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
        }
        reply.ok();
    }
}
