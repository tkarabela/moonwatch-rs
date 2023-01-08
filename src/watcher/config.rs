use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use regex::Regex;
use anyhow::{anyhow, bail, Result};
use json::JsonValue;
use crate::watcher::core::ActiveWindowEvent;

#[derive(Debug)]
pub struct WindowEventMatcher {
    pub window_title_regex: Option<Regex>,
    pub process_path_regex: Option<Regex>
}

#[derive(Debug)]
pub struct ConfigTag {
    pub tag: String,
    pub matcher: WindowEventMatcher
}

impl WindowEventMatcher {
    pub fn matches(&self, e: &ActiveWindowEvent) -> bool {
        if let Some(tmp) = &self.window_title_regex {
            if tmp.is_match(e.window_title.as_str()) {
                return true;
            }
        }

        if let Some(tmp) = &self.process_path_regex {
            if let Some(path) = e.process_path.to_str() {
                if tmp.is_match(path) {
                    return true;
                }
            }
        }

        false
    }

    pub fn from_json(val: &JsonValue) -> Result<WindowEventMatcher> {
        if !val.is_object() {
            bail!("WindowEventMatcher definition should be an object, not {:?}", val);
        }

        let window_title = &val["window_title"];
        let process_path = &val["process_path"];

        let window_title_regex = if let Some(tmp) = window_title.as_str() {
            Some(Regex::new(tmp)?)
        } else { None };

        let process_path_regex = if let Some(tmp) = process_path.as_str() {
            Some(Regex::new(tmp)?)
        } else { None };

        Ok(WindowEventMatcher {
            window_title_regex: window_title_regex,
            process_path_regex: process_path_regex,
        })
    }
}

#[derive(Debug)]
pub struct Config {
    pub output_dir: PathBuf,
    pub sample_every: Duration,
    pub write_every: Duration,
    pub tags: Vec<ConfigTag>,
    pub ignore: Vec<WindowEventMatcher>,
    pub anonymize: Vec<WindowEventMatcher>,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Config> {
        let data = fs::read_to_string(path)?;
        let d = json::parse(data.as_str())?;

        let mut tags = Vec::<ConfigTag>::new();

        for (key, val) in d["tags"].entries() {
            let mut tag_definitions = Vec::<&JsonValue>::new();
            if val.is_array() {
                for tmp in val.members() {
                    tag_definitions.push(tmp);
                }
            } else if val.is_object() {
                tag_definitions.push(val);
            } else {
                anyhow!("cannot read definition of tag {}", key);
            }

            for tag_definition in tag_definitions {
                let matcher = WindowEventMatcher::from_json(tag_definition)?;

                tags.push( ConfigTag {
                    tag: key.to_string(),
                    matcher
                });
            }
        }

        let mut ignore = Vec::<WindowEventMatcher>::new();
        for val in d["ignore"].members() {
            ignore.push(WindowEventMatcher::from_json(val)?);
        }

        let mut anonymize = Vec::<WindowEventMatcher>::new();
        for val in d["anonymize"].members() {
            anonymize.push(WindowEventMatcher::from_json(val)?);
        }

        let output_dir = PathBuf::from(d["main"]["output_dir"].as_str().ok_or(anyhow!("cannot read output_dir"))?);
        let sample_every = Duration::from_secs_f32(d["main"]["sample_every_sec"].as_f32().ok_or(anyhow!("cannot read sample_every_sec"))?);
        let write_every = Duration::from_secs_f32(d["main"]["write_every_sec"].as_f32().ok_or(anyhow!("cannot read write_every_sec"))?);

        Ok(Config {
            output_dir,
            sample_every,
            write_every,
            tags,
            ignore,
            anonymize,
        })
    }
}
