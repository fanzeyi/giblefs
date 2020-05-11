use crate::fs::{FileAttrBuilder, ToFileAttr};
use crate::inode::{Ino, Inode};
use anyhow::{anyhow, Result};
use fuse::FileAttr;
use git2::{Blob, Commit, Object, Oid, Tree};
use std::convert::TryFrom;
use time::Timespec;

macro_rules! impl_types {
    ($type: ident, $smtype: ident) => {
        paste::item! {
            // (inode, commit hash, object)
            pub struct [<Git $type>]<'a>(Ino, Oid, $type<'a>);

            impl<'a> [<Git $type>]<'a> {
                pub fn inode(&self) -> Ino {
                    self.0
                }

                pub fn parent(&self) -> Oid {
                    self.1
                }
            }

            impl<'a> Inode for [<Git $type>]<'a> {
                fn ino(&self) -> Ino {
                    self.0
                }
            }

            impl<'a> AsRef<$type<'a>> for [<Git $type>]<'a> {
                fn as_ref(&self) -> &$type<'a> {
                    &self.2
                }
            }

            impl<'a> TryFrom<(Ino, Oid, Object<'a>)> for [<Git $type>]<'a> {
                type Error = anyhow::Error;

                fn try_from((ino, commit, object): (Ino, Oid, Object<'a>)) -> Result<[<Git $type>]<'a>> {
                    let result = object.[<into_ $smtype>]();

                    if let Ok($smtype) = result {
                        Ok([<Git $type>](ino, commit, $smtype))
                    } else {
                        Err(anyhow!(format!("not expected {}", stringify!($smtype))))
                    }
                }
            }
        }
    };
}

impl_types!(Commit, commit);

impl<'a> ToFileAttr for GitCommit<'a> {
    fn to_file_attr(&self, builder: FileAttrBuilder) -> FileAttr {
        builder
            .ino(self.ino())
            .directory()
            .time(Timespec::new(self.as_ref().time().seconds(), 0))
            .nlink(2)
            .build()
    }
}

impl_types!(Tree, tree);

impl<'a> ToFileAttr for GitTree<'a> {
    fn to_file_attr(&self, builder: FileAttrBuilder) -> FileAttr {
        builder
            .ino(self.ino())
            .directory()
            .nlink(2 + self.as_ref().len() as u32)
            .build()
    }
}

impl_types!(Blob, blob);

impl<'a> ToFileAttr for GitBlob<'a> {
    fn to_file_attr(&self, builder: FileAttrBuilder) -> FileAttr {
        builder
            .ino(self.ino())
            .file()
            .size(self.as_ref().size())
            .nlink(2)
            .build()
    }
}
