#[cfg(target_os = "linux")]
mod unix;
#[cfg(target_os = "macos")]
mod unix;
#[cfg(target_os = "windows")]
mod windows;

use crate::common::{LinkEntry, LinkStatus, Result};

trait SysOP {
    fn create(entry: &LinkEntry) -> Result<()>;
    fn remove(entry: &LinkEntry) -> Result<()>;
    fn check(entry: &LinkEntry) -> LinkStatus;
}

pub fn create(entry: &LinkEntry) -> Result<()> {
    #[cfg(target_os = "windows")]
    return windows::OP::create(entry);
    #[cfg(unix)]
    return unix::OP::create(entry);
}

pub fn remove(entry: &LinkEntry) -> Result<()> {
    #[cfg(target_os = "windows")]
    return windows::OP::remove(entry);
    #[cfg(unix)]
    return unix::remove(entry);
}

pub fn check(entry: &LinkEntry) -> LinkStatus {
    #[cfg(target_os = "windows")]
    return windows::OP::check(entry);
    #[cfg(unix)]
    return unix::check(entry);
}
