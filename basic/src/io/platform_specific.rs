//! OS-specific functionality.
#[cfg(unix)] pub mod unix;
#[cfg(windows)] pub mod windows;
#[cfg(target_os = "ios")] pub mod ios;
#[cfg(target_os = "linux")] pub mod linux;
#[cfg(target_os = "macos")] pub mod macos;


// #[cfg] attribute indicates conditional compilation: each of these modules exists only on some platforms.
