use std::fs;
use std::os::windows::fs as osfs;

use crate::common::{LinkEntry, LinkKind, LinkStatus, Result};
use crate::error::SymmanError;
use crate::platform::SysOP;
pub struct OP;

impl SysOP for OP {
    fn create(entry: &LinkEntry) -> Result<()> {
        let result = if entry.target_path.is_dir() {
            osfs::symlink_dir(&entry.target_path, &entry.link_path)
        } else {
            osfs::symlink_file(&entry.target_path, &entry.link_path)
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.raw_os_error() == Some(1314) {
                    Err(SymmanError::InsufficientPrivilege)
                } else {
                    Err(SymmanError::Io(e))
                }
            }
        }
    }

    fn remove(entry: &LinkEntry) -> Result<()> {
        let meta = entry.link_path.symlink_metadata()?;

        if !meta.is_symlink() {
            return Err(SymmanError::NotASymlink(entry.link_path.clone()));
        }

        match entry.kind {
            LinkKind::Dir => fs::remove_dir(&entry.link_path)?,
            LinkKind::File => fs::remove_file(&entry.link_path)?,
        }

        Ok(())
    }

    fn check(entry: &LinkEntry) -> LinkStatus {
        match entry.link_path.symlink_metadata() {
            Ok(meta) => {
                if !meta.is_symlink() {
                    return LinkStatus::NotASymlink;
                }

                if entry.link_path.exists() {
                    LinkStatus::Healthy
                } else {
                    LinkStatus::BrokenLink
                }
            }
            Err(_) => LinkStatus::LinkMissing,
        }
    }
}
