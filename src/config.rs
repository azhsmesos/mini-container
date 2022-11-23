use crate::errors::Errcode;
use crate::ipc::generate_socket_pair;
use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub fd: RawFd,
}

impl ContainerOpts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
    ) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
        let sockets = generate_socket_pair()?;
        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|c| CString::new(c).expect("cannot read arg"))
            .collect();
        let path = argv[0].clone();
        Ok((
            ContainerOpts {
                path,
                argv,
                uid,
                mount_dir,
                fd: sockets.1.clone(),
            },
            sockets,
        ))
    }
}
