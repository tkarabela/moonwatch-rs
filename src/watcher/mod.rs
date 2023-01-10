pub mod core;
pub mod platforms;
pub mod config;
use anyhow::Result;
use crate::watcher::config::Config;
use crate::watcher::core::Desktop;

pub fn get_desktop(config: &Config) -> Result<Box<dyn core::Desktop>> {
    #[cfg(unix)]
    fn get_desktop_impl(_config: &Config) -> Result<Box<dyn core::Desktop>> {
        // TODO support more UNIX platforms, possibly use config to request a particular impl.
        
        let desktop = Box::new(platforms::linux::GnomeDesktop);
        
        desktop.check_implementation_available()?;
        Ok(desktop)
    }

    #[cfg(windows)]
    fn get_desktop_impl(config: &Config) -> Result<Box<dyn core::Desktop>> {
        let desktop = Box::new(platforms::windows::WindowsDesktop);
        desktop.check_implementation_available()?;
        Ok(desktop)
    }

    get_desktop_impl(config)
}

pub fn get_signal_channel() -> Result<crossbeam_channel::Receiver<core::MoonwatcherSignal>> {
    #[cfg(unix)]
    fn get_signal_channel_impl() -> Result<crossbeam_channel::Receiver<core::MoonwatcherSignal>> {
        platforms::linux::get_signal_channel()
    }

    #[cfg(windows)]
    fn get_signal_channel_impl() -> Result<crossbeam_channel::Receiver<core::MoonwatcherSignal>> {
        platforms::windows::get_signal_channel()
    }

    get_signal_channel_impl()
}
