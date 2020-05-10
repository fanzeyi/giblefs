use crate::git::types::{GitBlob, GitCommit, GitTree};
use crate::inode::{Ino, InodeGen};
use anyhow::{anyhow, Result};
use bimap::BiMap;
use git2::{Object, ObjectType, Oid, Repository, RepositoryOpenFlags};
use log::debug;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::path::PathBuf;

mod types;

pub struct GitRepo {
    path: PathBuf,
    repo: Repository,

    inode_gen: InodeGen,
    // inode <=> (commit hash, object id)
    inode_map: BiMap<Ino, (Oid, Oid)>,
}

impl GitRepo {
    pub fn new<P: Into<PathBuf>>(path: P, inode_gen: InodeGen) -> Result<Self> {
        let path = path.into();
        let repo = Repository::open_ext::<_, OsString, _>(
            &path,
            RepositoryOpenFlags::NO_SEARCH,
            Vec::new(),
        )?;
        Ok(GitRepo {
            path,
            repo,
            inode_gen,
            inode_map: BiMap::new(),
        })
    }

    /// Get an object along with an inode number, assign one if it is not assigned already
    pub fn get_object(
        &mut self,
        commit: Oid,
        hash: Oid,
        kind: Option<ObjectType>,
    ) -> Result<(Ino, Oid, Object)> {
        debug!("looking up object: {}", hash);
        let object = self.repo.find_object(hash, kind)?;

        if let Some(ino) = self.inode_map.get_by_right(&(commit, hash)) {
            debug!("found object {} in inode cache with inode {:?}", hash, ino);
            Ok((*ino, commit, object))
        } else {
            let ino = self.inode_gen.next();
            self.inode_map.insert(ino, (commit, hash));
            debug!("assigning {} with inode {:?}", hash, ino);
            Ok((ino, commit, object))
        }
    }

    /// Get an object by directly looking up in inode cache
    pub fn get_object_by_inode(
        &self,
        ino: Ino,
        kind: Option<ObjectType>,
    ) -> Result<(Ino, Oid, Object)> {
        debug!("looking up object for inode: {:?}", ino);
        if let Some((commit, hash)) = self.inode_map.get_by_left(&ino) {
            debug!("found object {} for inode {:?}", hash, ino);
            let object = self.repo.find_object(*hash, kind)?;

            Ok((ino, *commit, object))
        } else {
            Err(anyhow!("inode not found"))
        }
    }

    pub fn get_tree_by_commit(&mut self, hash: Oid) -> Result<GitTree> {
        let commit = self.repo.find_commit(hash)?;
        let root_tree = commit.tree_id();
        drop(commit);
        self.get_tree(hash, root_tree)
    }

    pub fn get_tree(&mut self, commit: Oid, hash: Oid) -> Result<GitTree> {
        GitTree::try_from(self.get_object(commit, hash, Some(ObjectType::Tree))?)
    }

    pub fn get_tree_by_inode(&self, ino: Ino) -> Result<GitTree> {
        GitTree::try_from(self.get_object_by_inode(ino, Some(ObjectType::Tree))?)
    }

    pub fn get_blob(&mut self, commit: Oid, hash: Oid) -> Result<GitBlob> {
        GitBlob::try_from(self.get_object(commit, hash, Some(ObjectType::Blob))?)
    }

    pub fn get_blob_by_inode(&self, ino: Ino) -> Result<GitBlob> {
        GitBlob::try_from(self.get_object_by_inode(ino, Some(ObjectType::Blob))?)
    }
}
