use crate::errors::Errcode;
use nix::sys::mman::MsFlags;
use std::path::PathBuf;
use nix::mount::mount;

pub fn set_mount_point(mount_dir: &PathBuf) -> Result<(), Errcode> {
    log::debug!("Setting mount points ...");
    mount_directory(None, &PathBuf::from("/"), vec![MsFlags::MS_REC, MsFlags::MS_PRIVATE])?;
    Ok(())
}

pub fn clean_mounts(_root_path: &PathBuf) -> Result<(), Errcode> {
    Ok(())
}

pub fn mount_directory(
    path: Option<&PathBuf>,
    mount_point: &PathBuf,
    flags: Vec<MsFlags>,
) -> Result<(), Errcode> {
    let mut ms_flags = MsFlags::empty();
    for f in flags.iter() {
        ms_flags.insert(*f);
    }
    match mount::<PathBuf, PathBuf, PathBuf, PathBuf>(path, mount_point, None, ms_flags, None) {
        Ok(_) => Ok(()),
        Err(e) => {
            if let Some(p) = path {
                log::error!("Cannot mount {} to {}: {}", p.to_str().unwrap(), mount_point.to_str().unwrap(), e);
            } else {
                log::error!("Cannot remount {}: {}", mount_point.to_str().unwrap(), e);
            }
            Err(Errcode::MountsError(3))
        },
    }
}
