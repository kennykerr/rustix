//! Tests for [`rustix::system`].

#![cfg(feature = "system")]
#![cfg(not(any(windows, target_os = "wasi")))]

#[cfg(target_os = "linux")]
mod reboot;
#[cfg(linux_kernel)]
mod sysinfo;
mod uname;
