use crate::errors::Errcode;
use crate::ipc::{recv_boolean, send_boolean};
use nix::sched::{unshare, CloneFlags};
use nix::unistd::Pid;
use std::os::unix::io::RawFd;

pub fn user_namespace(fd: RawFd, uid: u32) -> Result<(), Errcode> {
    log::debug!("Setting up user namespace with UID {}", uid);
    let has_user_ns = match unshare(CloneFlags::CLONE_NEWUSER) {
        Ok(_) => true,
        Err(_) => false,
    };
    send_boolean(fd, has_user_ns)?;
    if recv_boolean(fd)? {
        return Err(Errcode::NamespaceError(0));
    }
    if has_user_ns {
        log::info!("User namespace set up!!!");
    } else {
        log::info!("User namespace not supported, continuing...");
    }

    Ok(())
}

pub fn handle_child_uid_map(pid: Pid, fd: RawFd) -> Result<(), Errcode> {
    if recv_boolean(fd)? {
    } else {
        log::info!("No user namespace set up from child process");
    }
    log::debug!("child UID/GID map done, sending signal to child to continue...");
    send_boolean(fd, false)
}
