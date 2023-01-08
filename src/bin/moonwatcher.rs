use std::process::{Command, Stdio};
use std::{io, thread, time};
use std::collections::LinkedList;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use anyhow::bail;
use chrono::{DateTime, Utc};
use moonwatch_rs::watcher;
use moonwatch_rs::watcher::core::{ActiveWindowEvent, Desktop};
use regex::Regex;
use moonwatch_rs::watcher::config::Config;
use anyhow::Result;

enum ActiveWindowEventResult {
    DesktopLocked,
    Window { e: ActiveWindowEvent }
}

fn get_window_event(desktop: &dyn Desktop) -> Result<ActiveWindowEventResult> {
    if desktop.is_screen_locked() {
        Ok(ActiveWindowEventResult::DesktopLocked)
    } else {
        let window = desktop.get_active_window()?;
        let idle_duration = desktop.get_idle_duration();
        let process_path = window.get_process_path()?;
        let window_title = window.get_title()?;

        let e = ActiveWindowEvent::new(idle_duration, window_title, process_path);
        Ok(ActiveWindowEventResult::Window { e })
    }
}

fn main() -> anyhow::Result<()> {
    let config_path = PathBuf::from(std::env::args().nth(1).unwrap_or("moonwatch.json".to_string()));
    println!("--- Moonwatch ---");
    println!("Configuration file: {:?}", config_path);
    let config = Config::from_file(config_path.as_path())?;
    println!("Read configuration: {:?}", config);

    let desktop = watcher::get_desktop();

    loop {
        let res = get_window_event(desktop.as_ref());
        match res {
            Ok(ActiveWindowEventResult::DesktopLocked) => {
                thread::sleep(Duration::from_secs(600));
            }
            Ok(ActiveWindowEventResult::Window { mut e }) => {
                // ignore?
                let should_ignore = config.ignore.iter().any(|m| m.matches(&e));
                let should_anonymize = config.anonymize.iter().any(|m| m.matches(&e));

                // assign tags
                for t in &config.tags {
                    if t.matcher.matches(&e) {
                        e.tags.push_back(t.tag.clone())
                    }
                }

                println!("Event (ignore={}, anonymize={}): {:?}", should_ignore, should_anonymize, e);
                thread::sleep(config.sample_every);
            }
            _ => {
                thread::sleep(Duration::from_secs(60));
            }

        }
    }
}
