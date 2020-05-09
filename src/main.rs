use anyhow::Result;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use structopt::StructOpt;

mod fs;
mod git;
mod inode;

use crate::git::GitRepo;

#[derive(StructOpt)]
struct Options {
    repo: PathBuf,
    mount: PathBuf,

    #[structopt(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    better_panic::install();
    env_logger::init();

    let stop = Arc::new(AtomicBool::new(false));

    ctrlc::set_handler({
        let stop = stop.clone();
        move || {
            stop.store(true, Ordering::SeqCst);
        }
    })?;

    let options = Options::from_args();

    let mount_options: Vec<&OsStr> = ["-o", "ro", "-o", "fsname=gilber"]
        .iter()
        .map(|x| x.as_ref())
        .collect();

    let fs = fs::GilberFS::new(options.repo)?;

    let _mount = unsafe { fuse::spawn_mount(fs, &options.mount, &mount_options)? };

    while !stop.load(Ordering::SeqCst) {}

    Ok(())
}
