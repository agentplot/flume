//! The guard (116, 108): a shell line read as the commands it runs, each
//! by its words, so `sluice submit`, `wt submit`, `wt merge` and `git
//! push` are found wherever the line runs them, after `&&`, in a
//! pipeline, a subshell or `bash -c`, behind variables and `env`, and
//! never in a word that only mentions them. `git merge` is never refused:
//! a conflict's fix merges the base into the place's branch (112).

/// The commands `line` runs, each as its words, quotes removed.
pub fn commands(line: &str) -> Vec<Vec<String>> {
    let mut commands = vec![];
    let mut words: Vec<String> = vec![];
    let mut word = String::new();
    let mut in_word = false;
    let mut chars = line.chars().peekable();
    let end_word = |word: &mut String, in_word: &mut bool, words: &mut Vec<String>| {
        if *in_word {
            words.push(std::mem::take(word));
            *in_word = false;
        }
    };
    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                in_word = true;
                for c in chars.by_ref() {
                    if c == '\'' {
                        break;
                    }
                    word.push(c);
                }
            }
            '"' => {
                in_word = true;
                while let Some(c) = chars.next() {
                    match c {
                        '"' => break,
                        '\\' => {
                            if let Some(next) = chars.next() {
                                word.push(next);
                            }
                        }
                        _ => word.push(c),
                    }
                }
            }
            '\\' => {
                in_word = true;
                if let Some(next) = chars.next() {
                    if next != '\n' {
                        word.push(next);
                    }
                }
            }
            '$' if chars.peek() == Some(&'(') => {
                chars.next();
                end_word(&mut word, &mut in_word, &mut words);
                if !words.is_empty() {
                    commands.push(std::mem::take(&mut words));
                }
            }
            ';' | '&' | '|' | '\n' | '(' | ')' | '`' | '{' | '}' => {
                end_word(&mut word, &mut in_word, &mut words);
                if !words.is_empty() {
                    commands.push(std::mem::take(&mut words));
                }
            }
            c if c.is_whitespace() => end_word(&mut word, &mut in_word, &mut words),
            _ => {
                in_word = true;
                word.push(c);
            }
        }
    }
    end_word(&mut word, &mut in_word, &mut words);
    if !words.is_empty() {
        commands.push(words);
    }
    commands
}

/// Whether `word` sets a variable for the command after it.
fn assignment(word: &str) -> bool {
    word.split_once('=').is_some_and(|(name, _)| {
        !name.is_empty()
            && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            && !name.starts_with(|c: char| c.is_ascii_digit())
    })
}

/// The command's program and its arguments, past the variables it sets and
/// the commands that only run another.
fn program(words: &[String]) -> Option<(String, &[String])> {
    let mut rest = words;
    loop {
        let (first, after) = rest.split_first()?;
        if assignment(first) {
            rest = after;
            continue;
        }
        let name = first.rsplit('/').next().unwrap_or(first).to_string();
        match name.as_str() {
            "env" | "sudo" | "command" | "exec" | "nohup" | "time" | "nice" | "builtin" => {
                rest = after;
                while rest
                    .first()
                    .is_some_and(|word| word.starts_with('-') || assignment(word))
                {
                    rest = &rest[1..];
                }
            }
            "timeout" => {
                rest = after;
                while rest.first().is_some_and(|word| word.starts_with('-')) {
                    rest = &rest[1..];
                }
                rest = rest.get(1..).unwrap_or_default();
            }
            _ => return Some((name, after)),
        }
    }
}

/// The first word of `args` that is no option, past the options of
/// `taking` that take a value.
fn subcommand<'a>(args: &'a [String], taking: &[&str]) -> Option<&'a str> {
    let mut rest = args;
    while let Some((first, after)) = rest.split_first() {
        if taking.contains(&first.as_str()) {
            rest = after.get(1..).unwrap_or_default();
        } else if first.starts_with('-') {
            rest = after;
        } else {
            return Some(first);
        }
    }
    None
}

/// The act flume refuses that the command `words` runs, if it runs one.
fn refused_act(words: &[String]) -> Option<String> {
    let (name, args) = program(words)?;
    match name.as_str() {
        "sh" | "bash" | "zsh" | "dash" | "ksh" => {
            let at = args.iter().position(|word| {
                word.starts_with('-') && !word.starts_with("--") && word.contains('c')
            })?;
            let script = args.get(at + 1)?;
            refused(script)
        }
        "eval" => refused(&args.join(" ")),
        "git" => {
            let taking = [
                "-C",
                "-c",
                "--git-dir",
                "--work-tree",
                "--namespace",
                "--exec-path",
                "--config-env",
            ];
            (subcommand(args, &taking) == Some("push")).then(|| "git push".to_string())
        }
        "sluice" => (subcommand(args, &["--repository", "-C"]) == Some("submit"))
            .then(|| "sluice submit".to_string()),
        "wt" => match subcommand(args, &["-C", "--config"]) {
            Some("submit") => Some("wt submit".to_string()),
            Some("merge") => Some("wt merge".to_string()),
            _ => None,
        },
        "wt-submit" => Some("wt submit".to_string()),
        _ => None,
    }
}

/// The act flume refuses that `line` runs, the first if it runs several:
/// `sluice submit`, `wt submit`, `wt merge` or `git push` (116).
pub fn refused(line: &str) -> Option<String> {
    commands(line).iter().find_map(|words| refused_act(words))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_acts_flume_submits_are_refused_wherever_a_line_runs_them() {
        for (line, act) in [
            ("git push", "git push"),
            ("git add -A && git push origin HEAD", "git push"),
            ("git -C ../other -c a.b=c push --force", "git push"),
            ("cd x; GIT_TRACE=1 git push", "git push"),
            ("echo hi | env -i FOO=1 /usr/bin/git push", "git push"),
            ("(cd sub && git push)", "git push"),
            ("bash -c 'git commit -m x && git push'", "git push"),
            ("echo $(git push)", "git push"),
            ("sluice submit main --no-wait --json", "sluice submit"),
            ("wt submit", "wt submit"),
            ("wt merge main", "wt merge"),
            ("wt-submit", "wt submit"),
            ("timeout 30 git push", "git push"),
        ] {
            assert_eq!(refused(line).as_deref(), Some(act), "{line}");
        }
    }

    #[test]
    fn a_merge_and_a_mention_are_not_refused() {
        for line in [
            "git fetch origin && git merge 4f2a9c1",
            "git merge origin/main",
            "echo 'git push is refused here'",
            "grep -n \"sluice submit\" README.md",
            "git log --grep push",
            "git commit -m 'push the parser on'",
            "sluice status --json",
            "wt list",
            "",
        ] {
            assert_eq!(refused(line), None, "{line}");
        }
    }
}
