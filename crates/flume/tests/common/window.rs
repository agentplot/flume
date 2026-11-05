//! A stand-in for a window program, or for the program that asks the
//! display for a window: a script of the test's own that records each
//! run's words, environment and process, and refuses, hangs or holds as
//! the test scripts it. It opens no window of the host's display.

use std::collections::BTreeMap;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

/// The variables the stand-in's own shell sets for itself, left out of
/// what it records as given.
const SHELLS_OWN: [&str; 4] = ["PWD", "OLDPWD", "SHLVL", "_"];

/// One run of the stand-in: its words, none of which holds a line's end,
/// the environment it was given, its process, and whether it ended.
#[derive(Clone, Debug)]
pub struct Run {
    pub words: Vec<String>,
    pub environment: BTreeMap<String, String>,
    pub pid: u32,
    pub ended: bool,
}

/// A stand-in program named `name` in a folder's `bin`, its runs
/// recorded under that folder by its name, so several stand-ins may share
/// one `bin`.
pub struct StandIn {
    pub program: PathBuf,
    records: PathBuf,
    control: PathBuf,
}

impl StandIn {
    /// The stand-in `name` in `folder`'s `bin`, which it makes; that `bin`
    /// may be put first on a `PATH`.
    pub fn new(folder: &Path, name: &str) -> StandIn {
        let bin = folder.join("bin");
        let records = folder.join("runs").join(name);
        let control = folder.join("control").join(name);
        for dir in [&bin, &records, &control] {
            std::fs::create_dir_all(dir).expect("the stand-in's folders");
        }
        let program = bin.join(name);
        let script = format!(
            r#"#!/bin/sh
control='{control}'
if [ -f "$control/refuse" ]; then
  cat "$control/refuse" >&2
  rm -f "$control/refuse"
  exit 1
fi
if [ -f "$control/hang" ]; then
  rm -f "$control/hang"
  exec sleep 60
fi
records='{records}'
n=0
while ! mkdir "$records/lock.$n" 2>/dev/null; do n=$((n + 1)); done
mkdir "$records/new.$n"
env > "$records/new.$n/environment"
echo $$ > "$records/new.$n/pid"
printf '%s\n' "$@" > "$records/new.$n/words"
mv "$records/new.$n" "$records/run.$n"
if [ -f "$control/hold" ]; then
  i=0
  while [ ! -f "$control/release" ] && [ "$i" -lt 300 ]; do
    sleep 0.1
    i=$((i + 1))
  done
fi
: > "$records/run.$n/ended"
"#,
            control = control.display(),
            records = records.display(),
        );
        std::fs::write(&program, script).expect("the stand-in");
        std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755))
            .expect("the stand-in made runnable");
        StandIn {
            program,
            records,
            control,
        }
    }

    /// The folder the stand-in is in, for a `PATH`.
    pub fn bin(&self) -> PathBuf {
        self.program.parent().expect("the stand-in's folder").into()
    }

    /// The program's path as a word.
    pub fn path(&self) -> String {
        self.program.to_string_lossy().into()
    }

    /// The next run exits 1, saying `words`, and records nothing.
    pub fn refuse_next(&self, words: &str) {
        std::fs::write(self.control.join("refuse"), words).expect("the refusal scripted");
    }

    /// The next run never exits, and records nothing.
    pub fn hang_next(&self) {
        std::fs::write(self.control.join("hang"), "").expect("the hang scripted");
    }

    /// Every run from now waits after recording until [`StandIn::release`],
    /// or for thirty seconds.
    pub fn hold(&self) {
        std::fs::write(self.control.join("hold"), "").expect("the hold scripted");
    }

    /// Every held run ends.
    pub fn release(&self) {
        std::fs::write(self.control.join("release"), "").expect("the release");
    }

    /// The runs recorded so far, in order.
    pub fn runs(&self) -> Vec<Run> {
        let mut runs: Vec<(usize, Run)> = std::fs::read_dir(&self.records)
            .expect("the stand-in's records")
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                let n = name.strip_prefix("run.")?.parse().ok()?;
                Some((n, read_run(&entry.path())))
            })
            .collect();
        runs.sort_by_key(|(n, _)| *n);
        runs.into_iter().map(|(_, run)| run).collect()
    }

    /// The runs once no new one has come for half a second, within five,
    /// for a program its caller does not wait on.
    pub fn runs_settled(&self) -> Vec<Run> {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut seen = self.runs().len();
        let mut still = Instant::now();
        while Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(50));
            let now = self.runs().len();
            if now != seen {
                seen = now;
                still = Instant::now();
            } else if still.elapsed() >= Duration::from_millis(500) {
                break;
            }
        }
        self.runs()
    }

    /// The runs once `count` have come, within ten seconds.
    pub fn runs_at_least(&self, count: usize) -> Vec<Run> {
        let deadline = Instant::now() + Duration::from_secs(10);
        while self.runs().len() < count && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(50));
        }
        self.runs()
    }
}

/// The run recorded in `dir`.
fn read_run(dir: &Path) -> Run {
    let read = |name: &str| std::fs::read_to_string(dir.join(name)).unwrap_or_default();
    let words = read("words");
    Run {
        words: words
            .lines()
            .filter(|word| !word.is_empty())
            .map(str::to_string)
            .collect(),
        environment: read("environment")
            .lines()
            .filter_map(|line| line.split_once('='))
            .filter(|(name, _)| !SHELLS_OWN.contains(name))
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect(),
        pid: read("pid").trim().parse().unwrap_or_default(),
        ended: dir.join("ended").exists(),
    }
}

/// The words a POSIX shell reads `line` as, `sh` unquoting it.
pub fn shell_words(line: &str) -> Vec<String> {
    let output = Command::new("/bin/sh")
        .args([
            "-c",
            "eval \"set -- $1\"; printf '%s\\n' \"$@\"",
            "sh",
            line,
        ])
        .output()
        .expect("sh reads the line");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|word| !word.is_empty())
        .map(str::to_string)
        .collect()
}
