# Moonwatch.rs

## The `moonwatcher` daemon

### Installation

Tested on Ubuntu 22.04 LTS.

- Clone the repository.
- `./build.py && ./build/install_unix.py`
- This will install into `~/.moonwatcher-rs`.
  - It sets up a Systemd user service `moonwatcher-rs` that starts `moonwatcher` on startup.
  - Events are written to `~/.moonwatcher-rs/logs`
  - To customize, edit `~/.moonwatcher-rs/config.json`
  - To check up on the daemon, run `systemctl --user status moonwatch-rs`
  - To reload config, run `systemctl --user reload moonwatch-rs`

Supported platforms:

- Linux (and other unix-like systems), GNOME, X11
  - dependencies: `gnome-screensaver-command`, `xprintidle`, `xdotool`
  - tested on Ubuntu 22.04 LTS

### CLI

```sh
moonwatcher config.json
```

### JSON configuration

The overall structure is as follows:

- `"main"` (object)
  - `"output_dir"` (string)
    - path to directory where event logs are stored (relative to directory where the JSON config file is located)
  - `"sample_every_sec"` (number)
    - delay between sampling (seconds)
  - `"write_every_sec"` (number)
    - delay between writing samples to a file (seconds)
- `"ignore"` (object, array or null)
  - one or more `WindowEventMatcher` objects (see below)
  - events that match will not be recorded at all
- `"anonymize"` (object, array or null)
  - one or more `WindowEventMatcher` objects (see below)
  - events that match will be recorded in redacted from
- `"tags"` (object)
  - `"<tag name>"` (object, array or null)
    - one or more `WindowEventMatcher` objects (see below)
    - events that match will get assigned `"<tag name>"` in output

A `WindowEventMatcher` definition is an object with at least one of the following keys:

- `"window_title"` (string)
  - a regular expression (`regex::Regex`) that is tested against window title
- `"process_path"` (string)
  - a regular expression (`regex::Regex`) that is tested against process path

The `WindowEventMatcher` definition is used to match events – an event must match
all predicates defined by given `WindowEventMatcher` (AND semantics). If you want
OR semantics, just define multiple `WindowEventMatcher`s.

Full configuration example:

```json
{
  "main": {
    "output_dir": ".",
    "sample_every_sec": 15,
    "write_every_sec": 21600
  },
  "ignore": [{
    "window_title": "title to ignore"
  }],
  "anonymize": [{
    "window_title": "title to anonymize"
  }],
  "tags": {
    "youtube": [{
        "window_title": "YouTube — Mozilla Firefox$",
        "process_name": "firefox(\\.exe)?$"
      },
      {
        "window_title": "YouTube — Mozilla Firefox$",
        "process_name": "chrome(\\.exe)?$"
      }
    ],
    "pycharm": {
      "process_path": "JetBrains/Toolbox/apps/PyCharm"
    },
    "clion": {
      "process_path": "JetBrains/Toolbox/apps/CLion"
    }
  }
}
```
