use std::collections::HashMap;
use std::io::Write;
use std::process::Command;

fn execute_shell(program: &str, args: &[&str]) -> String {
    let output = Command::new(program).args(args).output().unwrap();
    String::from_utf8(output.stdout)
        .expect("failed to get version from git working copy")
}

fn main() {
    let hash = execute_shell("git", &["rev-parse", "HEAD"]);
    let status = execute_shell("git", &["status"]);
    let status = parse_status(&status);
    let modified = [status.not_staged, status.staged].into_iter().flatten().collect::<Vec<_>>();
    std::fs::File::create("version_details.txt")
        .expect("failed to open version.txt for write")
        .write_all(modified.join("\n").as_bytes())
        .expect("failed to write version.txt");
    let report = format!(
        "{}-{}{}",
        chrono::Utc::now().format("%Y-%m-%d"),
        hash.trim().split_at(7).0,
        if modified.is_empty() { "" } else { "*" }
    );
    std::fs::File::create("version.txt")
        .expect("failed to open version.txt for write")
        .write_all(report.as_bytes())
        .expect("failed to write version.txt");
}

struct GitStatus {
    staged: Vec<String>,
    not_staged: Vec<String>,
    untracked: Vec<String>,
}

fn parse_status(status: &str) -> GitStatus {
    let mut paragraphs: Vec<Vec<String>> = vec![vec![]];
    for line in status.trim().lines() {
        if line.trim().is_empty() {
            paragraphs.push(vec![]);
        } else {
            paragraphs.last_mut().unwrap().push(line.to_owned());
        }
    }

    let (_, sections) = paragraphs.split_at_mut(1);

    let mut status = GitStatus {
        staged: vec![],
        not_staged: vec![],
        untracked: vec![],
    };

    let mut results = HashMap::new();
    for section in sections {
        let (key, content) = section.split_at(1);
        let content = content.iter().map(|it| it.trim()).clone().collect::<Vec<_>>();
        let key = key.first().cloned().unwrap();
        results.insert(key, content);
    }

    if let Some(results) = results.get("Changes to be committed:") {
        status.staged = results.iter()
            .skip(1)
            .map(|it| it.trim().to_owned())
            .collect();
    }
    if let Some(results) = results.get("Changes not staged for commit:") {
        status.not_staged = results.iter()
            .skip(2)
            .map(|it| it.trim().to_owned())
            .collect();
    }
    if let Some(results) = results.get("Untracked files:") {
        status.untracked = results.iter()
            .skip(1)
            .map(|it| it.trim().to_owned())
            .collect();
    }
    status
}