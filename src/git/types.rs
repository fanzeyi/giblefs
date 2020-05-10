use crate::fs::{FileAttrBuilder, ToFileAttr};
use crate::inode::{Ino, Inode};
use anyhow::{anyhow, Result};
use fuse::FileAttr;
use git2::{Blob, Commit, Object, Tree};
use std::convert::TryFrom;
use time::Timespec;

macro_rules! impl_types {
    ($type: ident, $smtype: ident) => {
        paste::item! {
            pub struct [<Git $type>]<'a>(Ino, $type<'a>);

            impl<'a> Inode for [<Git $type>]<'a> {
                fn ino(&self) -> Ino {
                    self.0
                }
            }

            impl<'a> AsRef<$type<'a>> for [<Git $type>]<'a> {
                fn as_ref(&self) -> &$type<'a> {
                    &self.1
                }
            }

            impl<'a> TryFrom<(Ino, Object<'a>)> for [<Git $type>]<'a> {
                type Error = anyhow::Error;

                fn try_from((ino, object): (Ino, Object<'a>)) -> Result<[<Git $type>]<'a>> {
                    let result = object.[<into_ $smtype>]();

                    if let Ok($smtype) = result {
                        Ok([<Git $type>](ino, $smtype))
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
    fn to_file_attr(&self) -> FileAttr {
        FileAttrBuilder::new()
            .ino(self.ino())
            .directory()
            .time(Timespec::new(self.1.time().seconds(), 0))
            .nlink(2)
            .build()
    }
}

impl_types!(Tree, tree);

impl<'a> ToFileAttr for GitTree<'a> {
    fn to_file_attr(&self) -> FileAttr {
        FileAttrBuilder::new()
            .ino(self.ino())
            .directory()
            .nlink(2 + self.1.len() as u32)
            .build()
    }
}

impl_types!(Blob, blob);

impl<'a> ToFileAttr for GitBlob<'a> {
    fn to_file_attr(&self) -> FileAttr {
        FileAttrBuilder::new()
            .ino(self.ino())
            .file()
            .size(self.as_ref().size())
            .nlink(2)
            .build()
    }
}
