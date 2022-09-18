#[cfg(target_os = "linux")]
pub mod linux {
    pub use tunio_linux::*;
}
#[cfg(target_os = "macos")]
pub mod utun;
#[cfg(target_os = "windows")]
pub mod wintun {
    pub use tunio_wintun::*;
}
