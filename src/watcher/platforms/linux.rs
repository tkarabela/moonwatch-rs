use std::process::{Command, Stdio};
use std::time::Duration;
use std::fs;
use std::path::{Path, PathBuf};
use crate::watcher::core::{Window, Desktop};
use anyhow::Result;

pub struct LinuxDesktop;
pub struct LinuxXWindow { window_id: u64 }

impl Desktop for LinuxDesktop {
    fn is_screen_locked(&self) -> bool {
        let output = Command::new("gnome-screensaver-command")
            .arg("-q")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output();

        match output {
            Ok(output_) => {
                let s = String::from_utf8(output_.stdout).unwrap();
                let locked = s.contains("is active");
                locked
            }
            Err(_) => false
        }
    }

    fn get_idle_duration(&self) -> Duration {
        let output = Command::new("xprintidle")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output();

        match output {
            Ok(output_) => {
                let s = String::from_utf8(output_.stdout).unwrap();
                let val: u64 = s.trim().parse().unwrap();
                Duration::from_millis(val)
            }
            Err(_) => Duration::from_millis(0)
        }
    }

    fn get_active_window(&self) -> Result<Box<dyn Window>> {
        let output = Command::new("xdotool")
            .arg("getactivewindow")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()?;

        let window_id = String::from_utf8(output.stdout)?.trim().parse::<u64>()?;
        Ok(Box::new(LinuxXWindow { window_id }))
    }
}

impl Window for LinuxXWindow {
    fn get_title(&self) -> Result<String> {
        let output = Command::new("xdotool")
            .arg("getwindowname")
            .arg(self.window_id.to_string().as_str())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()?;

        Ok(String::from_utf8(output.stdout)?.trim().into())
    }

    fn get_process_id(&self) -> Result<u64> {
        let output = Command::new("xdotool")
            .arg("getwindowpid")
            .arg(self.window_id.to_string().as_str())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()?;

        Ok(String::from_utf8(output.stdout)?.trim().parse::<u64>()?)
    }

    fn get_process_path(&self) -> Result<PathBuf> {
        let pid = self.get_process_id()?;
        Ok(fs::read_link(format!("/proc/{}/exe", pid))?)
    }
}
