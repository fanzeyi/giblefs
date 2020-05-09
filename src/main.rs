use anyhow::Result;
use clap::clap_app;
use std::ffi::OsStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod fs;

fn main() -> Result<()> {
    env_logger::init();

    let stop = Arc::new(AtomicBool::new(false));

    ctrlc::set_handler({
        let stop = stop.clone();
        move || {
            stop.store(true, Ordering::SeqCst);
        }
    })?;

    let matches = clap_app!(gilber =>
        (about: "Git Filesystem")
        (@arg REPO: -r --repo +required "Path to repository to mount")
        (@arg MOUNT: +required "Path to mount the filesystem")
    )
    .get_matches();

    let options: Vec<&OsStr> = ["-o", "ro", "-o", "fsname=gilber"]
        .iter()
        .map(|x| x.as_ref())
        .collect();

    let _mount = unsafe {
        fuse::spawn_mount(
            fs::GilberFS {},
            &matches.value_of("MOUNT").unwrap(),
            &options,
        )?
    };

    while !stop.load(Ordering::SeqCst) {}

    Ok(())
}
