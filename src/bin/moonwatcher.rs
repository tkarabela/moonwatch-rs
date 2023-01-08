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
use signal_hook::{consts::SIGINT, iterator::Signals};
use signal_hook::consts::{SIGHUP, TERM_SIGNALS};

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

#[derive(Debug)]
enum MoonwatcherSignal {
    ReloadConfig,
    Terminate
}

fn signal_channel() -> Result<crossbeam_channel::Receiver<MoonwatcherSignal>> {
    let (sender, receiver) = crossbeam_channel::bounded(100);

    let mut sigs = vec![SIGHUP];
    sigs.extend(TERM_SIGNALS);
    let mut signals = Signals::new(sigs)?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received OS signal {:?}", sig);
            let moonwatcher_sig = match sig {
                SIGHUP => MoonwatcherSignal::ReloadConfig,
                _ => MoonwatcherSignal::Terminate
            };
            sender.send(moonwatcher_sig);
        }
    });

    Ok(receiver)
}

fn main() -> anyhow::Result<()> {
    let config_path = PathBuf::from(std::env::args().nth(1).unwrap_or("moonwatch.json".to_string()));
    println!("--- Moonwatch ---");
    println!("Configuration file: {:?}", config_path);
    let mut config = Config::from_file(config_path.as_path())?;
    println!("Read configuration: {:?}", config);

    let desktop = watcher::get_desktop();

    let signal_chan = signal_channel()?;
    let mut writer_tick_chan = crossbeam_channel::tick(config.write_every);
    let mut sample_tick_slow = false;
    let mut sample_tick_chan = crossbeam_channel::tick(config.sample_every);

    loop {
        crossbeam_channel::select! {
            recv(signal_chan) -> sig => {
                match sig? {
                    MoonwatcherSignal::ReloadConfig => {
                        println!("Reloading configuration file");
                        let new_config = Config::from_file(config_path.as_path());
                        match Config::from_file(config_path.as_path()) {
                            Ok(new_config) => {
                                println!("Read configuration: {:?}", new_config);
                                config = new_config;
                                sample_tick_slow = false;
                                sample_tick_chan = crossbeam_channel::tick(config.sample_every);
                                writer_tick_chan = crossbeam_channel::tick(config.write_every);
                            }
                            Err(e) => {
                                println!("Failed to reload configuration: {:?}", e);
                            }
                        }
                    }
                    MoonwatcherSignal::Terminate => {
                        println!("TODO writing data..."); // TODO
                        println!("Terminating due to OS signal");
                        break;
                    }
                    _ => bail!("unhandled MoonwatcherSignal")
                }
            }
            recv(writer_tick_chan) -> _ => {
                println!("TODO writing data..."); // TODO
            }
            recv(sample_tick_chan) -> _ => {
                let res = get_window_event(desktop.as_ref());
                match res {
                    Ok(ActiveWindowEventResult::DesktopLocked) => {
                        if !sample_tick_slow {
                            println!("slowing down sample rate");
                            sample_tick_slow = true;
                            sample_tick_chan = crossbeam_channel::tick(10*config.sample_every);
                        }
                    }
                    Ok(ActiveWindowEventResult::Window { mut e }) => {
                        // reset sample rate
                        if sample_tick_slow {
                            println!("resetting sample rate");
                            sample_tick_slow = false;
                            sample_tick_chan = crossbeam_channel::tick(config.sample_every);
                        }

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
                        if !sample_tick_slow {
                            println!("slowing down sample rate");
                            sample_tick_slow = true;
                            sample_tick_chan = crossbeam_channel::tick(10*config.sample_every);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
