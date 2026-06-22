use std::fs::{self, OpenOptions};
use std::io::{Write};

#[cfg(target_os = "windows")]
const HOSTS_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";
#[cfg(target_os = "linux")]
const HOSTS_PATH: &str = "/etc/hosts";

const START_MARKER: &str = "# NET_SENTINEL_START";
const END_MARKER: &str = "# NET_SENTINEL_END";

pub fn block_domains(file_path: &str) {
    let domains = fs::read_to_string(file_path).expect("File could not be read");
    let mut hosts = OpenOptions::new()
        .append(true)
        .open(HOSTS_PATH)
        .expect("Error: Could not open 'hosts' file. Run with administrator privileges!");

    writeln!(hosts, "{}", START_MARKER).unwrap();
    for domain in domains.lines() {
        writeln!(hosts, "127.0.0.1 {}", domain).unwrap();
    }
    writeln!(hosts, "{}", END_MARKER).unwrap();
    println!("Domains blocked!")
}

pub fn restore_hosts() {
    let content = fs::read_to_string(HOSTS_PATH).unwrap();
    let mut new_content = std::string::String::new();
    let mut skip = false;

    for line in content.lines() {
        if line == START_MARKER {
            skip = true;
        }
        if !skip {
            new_content.push_str(line);
            new_content.push('\n');
        }
        if line == END_MARKER {
            skip = false;
        }
    }
    fs::write(HOSTS_PATH, new_content).unwrap();
    println!("The hosts file has been restored to its original state!")
}
