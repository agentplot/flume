//! A program nix wrapped: `bin/claude` a wrapper that sets variables and
//! runs the program at `bin/.claude` (or `bin/.claude-wrapped`) in its
//! place. A terminal that starts an agent program by its kind sees the
//! wrapped program's own name, `.claude`, which no terminal takes for the
//! kind, and the program the settings name is not the one that runs
//! (record 0018; C.5). So the profile runs the wrapped program itself,
//! with the variables the wrapper sets, and the terminal runs it under
//! the kind's name.
//!
//! Both of nixpkgs' wrappers are read: makeCWrapper's binary, which
//! carries the command that generated it, `makeCWrapper '<program>' \`
//! then one `--set` or `--set-default` per line; and makeWrapper's shell
//! script, `export` lines then `exec [-a <name>] "<program>" "$@"`. A
//! wrapper doing anything else is not read as one, and the program the
//! settings name runs as named.

use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

/// The most of a file read for a wrapper: a wrapper is a few dozen
/// kilobytes, the program it wraps hundreds of megabytes.
const MOST: u64 = 1 << 20;

/// What a nix wrapper runs and the variables it sets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wrapped {
    /// The program the wrapper runs in its place.
    pub program: PathBuf,
    /// Each variable the wrapper sets, over any value given.
    pub set: BTreeMap<String, String>,
    /// Each variable the wrapper sets where none is given.
    pub defaults: BTreeMap<String, String>,
}

impl Wrapped {
    /// `env` with the wrapper's variables: each it sets over the value
    /// given, each it defaults where none is.
    pub fn environment(&self, env: &mut BTreeMap<String, String>) {
        for (name, value) in &self.defaults {
            env.entry(name.clone()).or_insert_with(|| value.clone());
        }
        for (name, value) in &self.set {
            env.insert(name.clone(), value.clone());
        }
    }
}

/// The program `program` names, by its path or on `path`'s folders.
pub fn located(program: &str, path: &str) -> Option<PathBuf> {
    if program.contains('/') {
        return Some(PathBuf::from(program));
    }
    std::env::split_paths(path)
        .map(|dir| dir.join(program))
        .find(|candidate| candidate.is_file())
}

/// What the nix wrapper at `file` runs and sets; none where `file` is no
/// wrapper this reads, or the program it names is not there.
pub fn unwrapped(file: &Path) -> Option<Wrapped> {
    let mut bytes = vec![];
    fs::File::open(file)
        .ok()?
        .take(MOST + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    if bytes.len() as u64 > MOST {
        return None;
    }
    let text = String::from_utf8_lossy(&bytes);
    let wrapped = if text.starts_with("#!") {
        shell_wrapper(&text)?
    } else {
        c_wrapper(&text)?
    };
    wrapped.program.is_file().then_some(wrapped)
}

/// makeCWrapper's command, as its binary carries it.
fn c_wrapper(text: &str) -> Option<Wrapped> {
    const MARK: &str = "makeCWrapper '";
    let at = text.find(MARK)?;
    let mut command = String::new();
    for line in text[at..].lines() {
        let line = line.trim_end();
        match line.strip_suffix('\\') {
            Some(continued) => {
                command.push_str(continued);
                command.push(' ');
            }
            None => {
                command.push_str(line);
                break;
            }
        }
    }
    let words = quoted_words(&command)?;
    let mut words = words.into_iter();
    if words.next()? != "makeCWrapper" {
        return None;
    }
    let mut wrapped = Wrapped {
        program: PathBuf::from(words.next()?),
        set: BTreeMap::new(),
        defaults: BTreeMap::new(),
    };
    while let Some(flag) = words.next() {
        match flag.as_str() {
            "--set" => {
                let name = words.next()?;
                wrapped.set.insert(name, words.next()?);
            }
            "--set-default" => {
                let name = words.next()?;
                wrapped.defaults.insert(name, words.next()?);
            }
            "--inherit-argv0" | "--resolve-argv0" => {}
            "--argv0" => {
                words.next()?;
            }
            _ => return None,
        }
    }
    Some(wrapped)
}

/// makeWrapper's script: a shebang, `export` lines, and its `exec`.
fn shell_wrapper(text: &str) -> Option<Wrapped> {
    let mut set = BTreeMap::new();
    let mut defaults = BTreeMap::new();
    let mut program = None;
    for line in text.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if program.is_some() {
            return None;
        }
        if let Some(assignment) = line.strip_prefix("export ") {
            let (name, value) = assignment.split_once('=')?;
            if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return None;
            }
            let default = format!("${{{name}-");
            match value.strip_prefix(&default) {
                Some(rest) => {
                    let value = rest.strip_suffix('}')?;
                    defaults.insert(name.to_string(), single(quoted_words(value)?)?);
                }
                None => {
                    set.insert(name.to_string(), single(quoted_words(value)?)?);
                }
            }
            continue;
        }
        let words = quoted_words(line.strip_prefix("exec ")?)?;
        let words: Vec<&str> = match words.first().map(String::as_str) {
            Some("-a") => words.iter().skip(2).map(String::as_str).collect(),
            _ => words.iter().map(String::as_str).collect(),
        };
        let [wrapped, "$@"] = words.as_slice() else {
            return None;
        };
        program = Some(PathBuf::from(wrapped));
    }
    Some(Wrapped {
        program: program?,
        set,
        defaults,
    })
}

fn single(words: Vec<String>) -> Option<String> {
    let [word] = <[String; 1]>::try_from(words).ok()?;
    Some(word)
}

/// The words of `text` as a shell reads single- and double-quoted words
/// with no expansion, `'\''` among them; none where a quote is open or a
/// word expands anything but `"$@"`.
fn quoted_words(text: &str) -> Option<Vec<String>> {
    let mut words = vec![];
    let mut word = String::new();
    let mut in_word = false;
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                in_word = true;
                loop {
                    match chars.next()? {
                        '\'' => break,
                        c => word.push(c),
                    }
                }
            }
            '"' => {
                in_word = true;
                loop {
                    match chars.next()? {
                        '"' => break,
                        '\\' => word.push(chars.next()?),
                        c => word.push(c),
                    }
                }
            }
            '\\' => {
                in_word = true;
                word.push(chars.next()?);
            }
            c if c.is_whitespace() => {
                if in_word {
                    words.push(std::mem::take(&mut word));
                    in_word = false;
                }
            }
            c => {
                in_word = true;
                word.push(c);
            }
        }
    }
    if in_word {
        words.push(word);
    }
    if words.iter().any(|word| word.contains('$') && word != "$@") {
        return None;
    }
    Some(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(label: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("flume-wrapper-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("bin")).expect("a scratch folder");
        fs::write(dir.join("bin/.claude"), "the program").expect("the wrapped program");
        dir
    }

    #[test]
    fn a_c_wrapper_is_read_from_the_command_it_carries() {
        let dir = scratch("c");
        let program = dir.join("bin/.claude");
        let mut bytes = b"\x7fELF\0\0binary\0".to_vec();
        bytes.extend(
            format!(
                "# The C-code for this binary wrapper has been generated using the following command:\n\
                 makeCWrapper '{}' \\\n    --set 'DISABLE_AUTOUPDATER' '1' \\\n    --set-default 'MODE' 'quiet' \\\n    --set 'WORDS' 'it'\\''s here'\n\
                 # (Use `nix-shell -p makeBinaryWrapper` to get access to makeCWrapper in your shell)\n",
                program.display()
            )
            .as_bytes(),
        );
        bytes.extend(b"\0more binary\0");
        fs::write(dir.join("bin/claude"), bytes).expect("the wrapper");
        let wrapped = unwrapped(&dir.join("bin/claude")).expect("read as a wrapper");
        assert_eq!(wrapped.program, program);
        assert_eq!(
            wrapped.set,
            BTreeMap::from([
                ("DISABLE_AUTOUPDATER".into(), "1".into()),
                ("WORDS".into(), "it's here".into()),
            ])
        );
        assert_eq!(
            wrapped.defaults,
            BTreeMap::from([("MODE".into(), "quiet".into())])
        );
        let mut env = BTreeMap::from([
            ("MODE".to_string(), "loud".to_string()),
            ("DISABLE_AUTOUPDATER".to_string(), "0".to_string()),
        ]);
        wrapped.environment(&mut env);
        assert_eq!(env["MODE"], "loud");
        assert_eq!(env["DISABLE_AUTOUPDATER"], "1");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn a_shell_wrapper_is_read_from_its_exports_and_exec() {
        let dir = scratch("sh");
        let program = dir.join("bin/.claude");
        fs::write(
            dir.join("bin/claude"),
            format!(
                "#! /nix/store/x-bash/bin/bash -e\nexport DISABLE_AUTOUPDATER='1'\nexport MODE=${{MODE-'quiet'}}\nexec -a \"claude\" \"{}\"  \"$@\" \n",
                program.display()
            ),
        )
        .expect("the wrapper");
        let wrapped = unwrapped(&dir.join("bin/claude")).expect("read as a wrapper");
        assert_eq!(wrapped.program, program);
        assert_eq!(
            wrapped.set,
            BTreeMap::from([("DISABLE_AUTOUPDATER".into(), "1".into())])
        );
        assert_eq!(
            wrapped.defaults,
            BTreeMap::from([("MODE".into(), "quiet".into())])
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn a_program_that_is_no_wrapper_is_run_as_named() {
        let dir = scratch("none");
        let cases = [
            "#!/bin/sh\necho hello\nexec \"/bin/true\" \"$@\"\n".to_string(),
            "#!/bin/sh\nexport PATH=$PATH:/x\nexec \"/bin/true\" \"$@\"\n".to_string(),
            format!(
                "#!/bin/sh\nexec \"{}\" \"$@\"\nexit 1\n",
                dir.join("bin/.claude").display()
            ),
            format!(
                "makeCWrapper '{}' \\\n    --prefix 'PATH' ':' '/x'\n",
                dir.join("bin/.claude").display()
            ),
            "makeCWrapper '/nowhere/.claude'\n".to_string(),
            "the program itself".to_string(),
        ];
        for text in cases {
            fs::write(dir.join("bin/claude"), &text).expect("the file");
            assert_eq!(unwrapped(&dir.join("bin/claude")), None, "{text}");
        }
        assert_eq!(unwrapped(&dir.join("bin/missing")), None);
        let _ = fs::remove_dir_all(dir);
    }
}
