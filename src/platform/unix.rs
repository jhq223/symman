use std::fs;
use std::os::unix::fs as osfs;

use crate::common::{LinkEntry, LinkStatus, Result};
use crate::error::SymmanError;
use crate::platform::SysOP;

pub struct OP;

impl SysOP for OP {
    fn create(entry: &LinkEntry) -> Result<()> {
        osfs::symlink(&entry.target_path, &entry.link_path)?;

        Ok(())
    }

    fn remove(entry: &LinkEntry) -> Result<()> {
        let meta = entry.link_path.symlink_metadata()?;

        if !meta.is_symlink() {
            return Err(SymmanError::NotASymlink(entry.link_path.clone()));
        }
        fs::remove_file(&entry.link_path)?;
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
