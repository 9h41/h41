use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct PortEntry {
    pub pid: i64,
    pub name: String,
    pub args: Option<String>,
    pub listen: Vec<String>,
    pub cwd: Option<String>,
}

/// Check if the platform-specific port discovery tool is available.
pub fn is_available() -> bool {
    platform::is_available()
}

/// Retrieve all listening TCP ports.
pub fn all() -> Vec<PortEntry> {
    platform::all()
}

/// Kill a process by PID. Returns true on success.
pub fn kill_pid(pid: i64) -> bool {
    platform::kill_pid(pid)
}

/// Detect the current user's home directory prefix from a list of CWD paths.
pub fn detect_home_prefix(cwds: impl Iterator<Item = String>) -> Option<String> {
    platform::detect_home_prefix(cwds)
}

#[cfg(unix)]
mod platform {
    use super::PortEntry;
    use itertools::Itertools;
    use regex::Regex;
    use std::process::{Command, Output, Stdio};

    pub fn is_available() -> bool {
        Command::new("lsof")
            .arg("-v")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    }

    pub fn all() -> Vec<PortEntry> {
        let output = run_lsof();
        let raw = String::from_utf8_lossy(&output.stdout).to_string();
        parse_lsof_output(&raw)
    }

    pub fn kill_pid(pid: i64) -> bool {
        Command::new("kill")
            .arg(pid.to_string())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    pub fn detect_home_prefix(cwds: impl Iterator<Item = String>) -> Option<String> {
        let re = Regex::new(r"^(/(?:Users|home)/[^/]+)").ok()?;
        let homes: Vec<String> = cwds
            .filter_map(|cwd| re.captures(&cwd).map(|c| c[1].to_string()))
            .collect();
        most_frequent(homes)
    }

    fn run_lsof() -> Output {
        Command::new("lsof")
            .args(["-iTCP", "-sTCP:LISTEN", "-P", "-Fcn"])
            .output()
            .expect("failed to execute lsof")
    }

    pub(crate) fn parse_lsof_output(lsof_output: &str) -> Vec<PortEntry> {
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

    pub(crate) fn read_cwd_from_lsof_output(process_info: Option<&str>) -> Option<String> {
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

    fn most_frequent(items: Vec<String>) -> Option<String> {
        if items.is_empty() {
            return None;
        }
        let mut counts = std::collections::HashMap::new();
        for item in &items {
            *counts.entry(item.clone()).or_insert(0) += 1;
        }
        counts.into_iter().max_by_key(|(_, v)| *v).map(|(k, _)| k)
    }
}

#[cfg(windows)]
mod platform {
    use super::PortEntry;
    use itertools::Itertools;
    use regex::Regex;
    use std::collections::HashMap;
    use std::process::{Command, Stdio};

    pub fn is_available() -> bool {
        Command::new("netstat")
            .arg("-?")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    }

    pub fn all() -> Vec<PortEntry> {
        let netstat_output = Command::new("netstat")
            .args(["-ano", "-p", "TCP"])
            .output()
            .expect("failed to execute netstat");
        let raw = String::from_utf8_lossy(&netstat_output.stdout).to_string();
        parse_netstat_output(&raw)
    }

    pub fn kill_pid(pid: i64) -> bool {
        Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    pub fn detect_home_prefix(cwds: impl Iterator<Item = String>) -> Option<String> {
        // Match Windows user profile paths like C:\Users\username
        let re = Regex::new(r"(?i)^([a-z]:\\Users\\[^\\]+)").ok()?;
        let homes: Vec<String> = cwds
            .filter_map(|cwd| re.captures(&cwd).map(|c| c[1].to_string()))
            .collect();
        most_frequent(homes)
    }

    fn parse_netstat_output(output: &str) -> Vec<PortEntry> {
        // netstat -ano output looks like:
        //   TCP    0.0.0.0:8080           0.0.0.0:0              LISTENING       1234
        //   TCP    [::]:8080              [::]:0                 LISTENING       1234
        let line_re = Regex::new(r"(?i)^\s*TCP\s+(\S+)\s+\S+\s+LISTENING\s+(\d+)").unwrap();

        let mut pid_map: HashMap<i64, Vec<String>> = HashMap::new();

        for line in output.lines() {
            if let Some(caps) = line_re.captures(line) {
                let local_addr = caps[1].to_string();
                let pid: i64 = caps[2].parse().unwrap_or(0);
                if pid == 0 {
                    continue;
                }
                // Normalize address: [::]:port -> *:port, 0.0.0.0:port -> *:port
                let normalized = normalize_address(&local_addr);
                pid_map.entry(pid).or_default().push(normalized);
            }
        }

        pid_map
            .into_iter()
            .map(|(pid, addrs)| {
                let (name, args, cwd) = get_process_info(pid);
                PortEntry {
                    pid,
                    name,
                    args,
                    listen: addrs.into_iter().unique().collect(),
                    cwd,
                }
            })
            .collect()
    }

    fn normalize_address(addr: &str) -> String {
        let addr = addr.replace("[::1]", "localhost");
        let addr = addr.replace("[::]", "*");
        let addr = addr.replace("127.0.0.1", "localhost");
        addr.replace("0.0.0.0", "*")
    }

    fn get_process_info(pid: i64) -> (String, Option<String>, Option<String>) {
        // Use wmic to get process name, command line, and working directory
        let output = Command::new("wmic")
            .args([
                "process",
                "where",
                &format!("ProcessId={}", pid),
                "get",
                "Name,CommandLine,ExecutablePath",
                "/FORMAT:LIST",
            ])
            .output()
            .ok();

        let mut name = format!("PID {}", pid);
        let mut args = None;
        let mut cwd = None;

        if let Some(out) = output {
            let text = String::from_utf8_lossy(&out.stdout).to_string();
            for line in text.lines() {
                let line = line.trim();
                if let Some(val) = line.strip_prefix("Name=") {
                    if !val.is_empty() {
                        name = val.to_string();
                    }
                } else if let Some(val) = line.strip_prefix("CommandLine=") {
                    if !val.is_empty() {
                        args = Some(val.to_string());
                    }
                } else if let Some(val) = line.strip_prefix("ExecutablePath=") {
                    if !val.is_empty() {
                        // Use the executable's parent directory as CWD approximation
                        if let Some(parent) = std::path::Path::new(val).parent() {
                            cwd = Some(parent.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        (name, args, cwd)
    }

    fn most_frequent(items: Vec<String>) -> Option<String> {
        if items.is_empty() {
            return None;
        }
        let mut counts = std::collections::HashMap::new();
        for item in &items {
            *counts.entry(item.clone()).or_insert(0) += 1;
        }
        counts.into_iter().max_by_key(|(_, v)| *v).map(|(k, _)| k)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    use super::platform::{parse_lsof_output, read_cwd_from_lsof_output};

    #[cfg(unix)]
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

    #[cfg(unix)]
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

    #[test]
    fn should_detect_unix_home_prefix() {
        let cwds = vec![
            "/Users/cgatay/projects/foo".to_string(),
            "/Users/cgatay/projects/bar".to_string(),
            "/usr/local/bin".to_string(),
        ];
        let result = super::detect_home_prefix(cwds.into_iter());
        #[cfg(unix)]
        assert_eq!(result, Some("/Users/cgatay".to_string()));
        #[cfg(windows)]
        assert_eq!(result, None);
    }
}
