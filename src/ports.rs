use itertools::Itertools;
use regex::Regex;
use serde::Serialize;
use std::process::{Command, Output, Stdio};

#[derive(Serialize, Debug, Clone)]
pub struct PortEntry {
    pub pid: i64,
    pub name: String,
    pub args: Option<String>,
    pub listen: Vec<String>,
    pub cwd: Option<String>,
}

/// Check if `lsof` is available on the system.
pub fn is_available() -> bool {
    Command::new("lsof")
        .arg("-v")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

/// Retrieve all listening TCP ports via `lsof`.
pub fn all() -> Vec<PortEntry> {
    let output = run_lsof();
    let raw = String::from_utf8_lossy(&output.stdout).to_string();
    parse_lsof_output(&raw)
}

fn run_lsof() -> Output {
    Command::new("lsof")
        .args(["-iTCP", "-sTCP:LISTEN", "-P", "-Fcn"])
        .output()
        .expect("failed to execute lsof")
}

fn parse_lsof_output(lsof_output: &str) -> Vec<PortEntry> {
    let file_regex = Regex::new(r"^f[0-9]*$").unwrap();
    let chunks = lsof_output.split("\np");

    chunks
        .map(|s| {
            s.split('\n')
                .filter(|line| !file_regex.is_match(line))
                .collect::<Vec<&str>>()
        })
        .filter(|v| v.len() > 2)
        .map(|vec| {
            let pid = vec[0].replace('p', "").parse::<i64>().unwrap_or(0);
            PortEntry {
                pid,
                name: vec[1][1..].to_string(),
                args: get_args_for_pid(pid),
                listen: vec
                    .split_at(2)
                    .1
                    .iter()
                    .filter(|s| !s.is_empty())
                    .map(|s| s[1..].to_string())
                    .unique()
                    .collect(),
                cwd: get_cwd_for_pid(pid),
            }
        })
        .filter(|c| c.pid != 0)
        .collect()
}

fn get_args_for_pid(pid: i64) -> Option<String> {
    Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "args="])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

fn get_cwd_for_pid(pid: i64) -> Option<String> {
    let process_info = Command::new("lsof")
        .args(["-p", &pid.to_string(), "-Ffn"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned());
    read_cwd_from_lsof_output(process_info.as_deref())
}

fn read_cwd_from_lsof_output(process_info: Option<&str>) -> Option<String> {
    let pi = process_info?;
    let lines: Vec<&str> = pi.split('\n').collect();
    let fcwd_index = lines.iter().position(|s| s.starts_with("fcwd"))?;
    let cwd_line = lines.get(fcwd_index + 1)?;
    if !cwd_line.is_empty() {
        Some(cwd_line[1..].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_lsof_output() {
        let sample_output = "p35985\n\
            cShimo-Setapp\n\
            f52\n\
            n*:61600\n\
            p42166\n\
            cBetterTouchTool\n\
            f16\n\
            n*:57411\n\
            f17\n\
            n*:57411\n\
            p47915\n\
            cidea\n\
            f7\n\
            nlocalhost:10001\n\
            f226\n\
            nlocalhost:6942\n\
            f718\n\
            nlocalhost:17434\n\
            f737\n\
            nlocalhost:63342\n\
            f761\n\
            nlocalhost:9123";

        let parsed = parse_lsof_output(sample_output);
        assert_eq!(3, parsed.len());

        let first = &parsed[0];
        assert_eq!(35985, first.pid);
        assert_eq!("Shimo-Setapp", first.name);
        assert_eq!(1, first.listen.len());
        assert_eq!("*:61600", &first.listen[0]);

        let last = &parsed[2];
        assert_eq!(47915, last.pid);
        assert_eq!("idea", last.name);
        assert_eq!(5, last.listen.len());
        assert_eq!("localhost:10001", &last.listen[0]);
        assert_eq!("localhost:9123", &last.listen[4]);
    }

    #[test]
    fn should_parse_cwd_from_lsof() {
        let sample_output = "p47915\n\
            fcwd\n\
            n/Users/cgatay/Library/Application Support/JetBrains/Toolbox/apps/IDEA-U/ch-0/181.4445.4/IntelliJ IDEA 2018.1 EAP.app/Contents/bin\n\
            ftxt\n\
            n/Users/cgatay/Library/Application Support/JetBrains/Toolbox/apps/IDEA-U/ch-0/181.4445.4/IntelliJ IDEA 2018.1 EAP.app/Contents/MacOS/idea\n\
            ftxt\n\
            n/usr/share/icu/icudt59l.dat\n\
            ftxt\n\
            n/Users/cgatay/Library/Application Support/JetBrains/Toolbox/apps/IDEA-U/ch-0/181.4445.4/IntelliJ IDEA 2018.1 EAP.app/Contents/jdk/Contents/Home/jre/lib/jli/libjli.dylib\n";

        let maybe_cwd = read_cwd_from_lsof_output(Some(sample_output));
        assert!(maybe_cwd.is_some());
        assert_eq!(
            "/Users/cgatay/Library/Application Support/JetBrains/Toolbox/apps/IDEA-U/ch-0/181.4445.4/IntelliJ IDEA 2018.1 EAP.app/Contents/bin",
            maybe_cwd.unwrap()
        );
    }
}
