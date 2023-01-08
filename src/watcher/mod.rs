pub mod core;
pub mod platforms;
pub mod config;

pub fn get_desktop() -> Box<dyn core::Desktop> {
    #[cfg(target_os = "linux")]
    fn get_desktop_impl() -> Box<dyn core::Desktop> {
        Box::new(platforms::linux::LinuxDesktop)
    }

    #[cfg(target_os = "windows")]
    fn get_desktop_impl() -> Box<dyn core::Desktop> {
        Box::new(platforms::windows::WindowsDesktop)
    }

    get_desktop_impl()
}
