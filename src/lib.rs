use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Output};
use std::{fs, io};

pub fn grep(content: BufReader<File>, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        let line = line.unwrap();
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

pub fn cat(content: BufReader<File>, mut writer: impl std::io::Write) {
    for line in content.lines() {
        let line = line.unwrap();
        writeln!(writer, "{}", line).unwrap();
    }
}

pub fn head(content: BufReader<File>, mut writer: impl std::io::Write) {
    for (i, line) in content.lines().enumerate() {
        if i < 10 {
            let line = line.unwrap();
            writeln!(writer, "{}", line).unwrap();
        } else {
            break;
        }
    }
}

pub fn ls(
    path: &std::path::PathBuf,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    for entry in entries {
        writeln!(writer, "{}", entry.to_str().unwrap()).unwrap();
    }

    Ok(())
}

pub fn whoami(mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    let output: Output = Command::new("whoami")
        .output()
        .expect("failed to execute process");
    writeln!(writer, "{}", String::from_utf8(output.stdout).unwrap()).unwrap();
    Ok(())
}
