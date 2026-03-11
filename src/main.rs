use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "jvs", version, about = "Java Version Switch - manage Java versions on macOS and Linux")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all installed Java versions
    List,
    /// Show the currently active Java version
    Current,
    /// Switch to a specified Java version (prefix match)
    Use {
        /// Version prefix to match, e.g. "17", "11", "1.8"
        version: String,
    },
}

#[derive(Debug)]
struct JavaVersion {
    version: String,
    vendor: String,
    home: PathBuf,
}

#[derive(Serialize, Deserialize, Default)]
struct JvsConfig {
    current: Option<String>,
}

fn jvs_dir() -> PathBuf {
    dirs::home_dir().expect("Cannot determine home directory").join(".jvs")
}

fn config_path() -> PathBuf {
    jvs_dir().join("config.json")
}

fn env_path() -> PathBuf {
    jvs_dir().join("env")
}

fn load_config() -> JvsConfig {
    fs::read_to_string(config_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_config(config: &JvsConfig) {
    let dir = jvs_dir();
    fs::create_dir_all(&dir).expect("Failed to create ~/.jvs directory");
    let json = serde_json::to_string_pretty(config).expect("Failed to serialize config");
    fs::write(config_path(), json).expect("Failed to write config");
}

fn parse_release_file(home: &Path) -> Option<(String, String)> {
    let content = fs::read_to_string(home.join("release")).ok()?;
    let mut version = None;
    let mut vendor = None;
    for line in content.lines() {
        if let Some(val) = line.strip_prefix("JAVA_VERSION=") {
            version = Some(val.trim_matches('"').to_string());
        } else if let Some(val) = line.strip_prefix("IMPLEMENTOR=") {
            vendor = Some(val.trim_matches('"').to_string());
        }
    }
    Some((version?, vendor.unwrap_or_else(|| "Unknown".into())))
}

fn scan_dirs() -> Vec<PathBuf> {
    let home = dirs::home_dir();
    if cfg!(target_os = "macos") {
        vec![
            PathBuf::from("/Library/Java/JavaVirtualMachines"),
            home.map(|h| h.join("Library/Java/JavaVirtualMachines"))
                .unwrap_or_default(),
        ]
    } else {
        // Linux
        let mut dirs = vec![
            PathBuf::from("/usr/lib/jvm"),
            PathBuf::from("/usr/java"),
            PathBuf::from("/opt/java"),
        ];
        if let Some(h) = home {
            dirs.push(h.join(".sdkman/candidates/java"));
            dirs.push(h.join(".jdks"));
        }
        dirs
    }
}

fn try_resolve_home(entry_path: &Path) -> Option<PathBuf> {
    // macOS: JDK_DIR/Contents/Home/release
    let mac_home = entry_path.join("Contents/Home");
    if mac_home.join("release").exists() {
        return Some(mac_home);
    }
    // Linux / flat layout: JDK_DIR/release
    if entry_path.join("release").exists() {
        return Some(entry_path.to_path_buf());
    }
    None
}

fn detect_java_versions() -> Vec<JavaVersion> {
    let mut versions = Vec::new();
    for dir in &scan_dirs() {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            if let Some(home) = try_resolve_home(&entry.path()) {
                if let Some((ver, ven)) = parse_release_file(&home) {
                    versions.push(JavaVersion {
                        version: ver,
                        vendor: ven,
                        home,
                    });
                }
            }
        }
    }
    versions.sort_by(|a, b| b.version.cmp(&a.version));
    versions
}

fn get_current_home() -> Option<String> {
    // Priority: jvs config > JAVA_HOME env
    load_config()
        .current
        .or_else(|| std::env::var("JAVA_HOME").ok())
}

fn cmd_list() {
    let versions = detect_java_versions();
    if versions.is_empty() {
        println!("{}", "No Java versions found.".yellow());
        return;
    }
    let current = get_current_home();
    println!("{}", "Installed Java versions:".bold());
    for jv in &versions {
        let home_str = jv.home.display().to_string();
        let is_current = current.as_deref() == Some(&home_str);
        let marker = if is_current {
            " *".green().bold().to_string()
        } else {
            "  ".to_string()
        };
        let ver_display = if is_current {
            jv.version.green().bold().to_string()
        } else {
            jv.version.clone()
        };
        println!(
            "{} {:<12} ({})  {}",
            marker,
            ver_display,
            jv.vendor.dimmed(),
            home_str.dimmed()
        );
    }
}

fn cmd_current() {
    let versions = detect_java_versions();
    let current = get_current_home();
    match current {
        Some(home) => {
            let info = versions.iter().find(|v| v.home.display().to_string() == home);
            match info {
                Some(jv) => println!(
                    "Current Java: {} ({}) \n  JAVA_HOME={}",
                    jv.version.green().bold(),
                    jv.vendor,
                    jv.home.display()
                ),
                None => println!("JAVA_HOME={}\n{}", home, "(not managed by jvs)".yellow()),
            }
        }
        None => println!("{}", "No Java version is currently set.".yellow()),
    }
}

fn cmd_use(version_prefix: &str) {
    let versions = detect_java_versions();
    let matches: Vec<&JavaVersion> = versions
        .iter()
        .filter(|v| v.version.starts_with(version_prefix))
        .collect();

    match matches.len() {
        0 => {
            eprintln!(
                "{} No installed Java version matches '{}'",
                "Error:".red().bold(),
                version_prefix
            );
            eprintln!("Run `jvs list` to see available versions.");
            std::process::exit(1);
        }
        1 => {
            let selected = matches[0];
            let home_str = selected.home.display().to_string();

            // Write env file - clean old Java paths from PATH before adding new one
            let env_content = format!(
                r#"export JAVA_HOME="{home}"
export PATH="$(echo "$PATH" | tr ':' '\n' | grep -v -E '/JavaVirtualMachines/|/usr/lib/jvm/|/usr/java/|/opt/java/|/\.sdkman/candidates/java/|/\.jdks/' | tr '\n' ':' | sed 's/:$//')"
export PATH="$JAVA_HOME/bin:$PATH"
"#,
                home = home_str
            );
            let dir = jvs_dir();
            fs::create_dir_all(&dir).expect("Failed to create ~/.jvs");
            fs::write(env_path(), env_content).expect("Failed to write env file");

            // Save config
            save_config(&JvsConfig {
                current: Some(home_str.clone()),
            });

            println!(
                "{} Switched to Java {} ({})",
                "✓".green().bold(),
                selected.version.green().bold(),
                selected.vendor
            );
            println!("  JAVA_HOME={}", home_str);
            println!(
                "\n  Run {} to apply in current shell.",
                "source ~/.jvs/env".cyan()
            );
        }
        _ => {
            eprintln!(
                "{} Multiple versions match '{}'. Be more specific:",
                "Error:".red().bold(),
                version_prefix
            );
            for m in &matches {
                eprintln!("  - {} ({})", m.version, m.vendor);
            }
            std::process::exit(1);
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::List => cmd_list(),
        Commands::Current => cmd_current(),
        Commands::Use { version } => cmd_use(&version),
    }
}
