use crate::git::GitRepo;
use crate::inode::InodeGen;
use anyhow::Result;
use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use git2::Oid;
use lazy_static::lazy_static;
use libc::ENOENT;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::path::PathBuf;
use time::Timespec;

pub mod attr;

pub use attr::{FileAttrBuilder, ToFileAttr};

lazy_static! {
    static ref UNIX_EPOCH: Timespec = Timespec::new(0, 0);
    static ref TTL: Timespec = Timespec::new(1, 0);
    static ref ROOT_ATTR: FileAttr = FileAttr {
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
}

pub struct GilberFS {
    repo: GitRepo,
}

impl GilberFS {
    pub fn new(repo: PathBuf) -> Result<Self> {
        Ok(GilberFS {
            repo: GitRepo::new(repo, InodeGen::new())?,
        })
    }

    fn lookup_commit(&mut self, hash: &str) -> Result<FileAttr> {
        let oid = Oid::from_str(hash)?;
        let commit = self.repo.get_commit(oid)?;
        Ok(commit.to_file_attr())
    }
}

impl Filesystem for GilberFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 {
            // looking up by commit hash
            if let Some(hash) = name.to_str() {
                if let Ok(attr) = self.lookup_commit(hash) {
                    reply.entry(&TTL, &attr, 0);
                    return;
                }
            }
        }

        reply.error(ENOENT);
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        if ino == 1 {
            reply.attr(&TTL, &ROOT_ATTR);
        } else if let Ok(commit) = self.repo.get_commit_by_inode(ino.into()) {
            reply.attr(&TTL, &commit.to_file_attr());
        } else if let Ok(tree) = self.repo.get_tree_by_inode(ino.into()) {
            reply.attr(&TTL, &tree.to_file_attr());
        } else if let Ok(blob) = self.repo.get_blob_by_inode(ino.into()) {
            reply.attr(&TTL, &blob.to_file_attr());
        } else {
            reply.error(ENOENT);
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        reply: ReplyData,
    ) {
        if ino == 1 {
            reply.error(libc::EISDIR);
        } else if let Ok(blob) = self.repo.get_blob_by_inode(ino.into()) {
            if let (Ok(offset), Ok(size)) = (usize::try_from(offset), usize::try_from(size)) {
                let content = blob.as_ref().content();
                reply.data(&content[offset..(offset + size)])
            } else {
                // offset or size is too big for us to handle
                reply.error(libc::EINVAL)
            }
        } else if let Ok(_) = self.repo.get_commit_by_inode(ino.into()) {
            reply.error(libc::EISDIR);
        } else if let Ok(_) = self.repo.get_tree_by_inode(ino.into()) {
            reply.error(libc::EISDIR);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        if ino == 1 {
            reply.error(ENOENT);
            return;
        }

        let maybe_tree_id = if let Ok(commit) = self.repo.get_commit_by_inode(ino.into()) {
            Some(commit.as_ref().tree_id())
        } else {
            None
        };

        if let Some(oid) = maybe_tree_id {
            let tree = self.repo.get_tree(oid);
            // todo

            let entries = vec![
                (1, FileType::Directory, "."),
                (1, FileType::Directory, ".."),
                (2, FileType::RegularFile, "hello.txt"),
            ];

            for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
                // i + 1 means the index of the next entry
                reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
            }
        }

        reply.error(ENOENT);
    }
}
