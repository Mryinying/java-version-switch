# JVS - Java Version Switch

A lightweight CLI tool written in Rust to manage and switch between multiple Java versions on macOS and Linux.

## Features

- List all installed Java versions on macOS and Linux
- Display the currently active Java version
- Switch between Java versions by setting JAVA_HOME
- Automatic PATH cleanup to prevent version conflicts
- Shell function wrapper for instant switching without manual source
- Supports Java installations in standard locations:
  - macOS: `/Library/Java/JavaVirtualMachines/`, `~/Library/Java/JavaVirtualMachines/`
  - Linux: `/usr/lib/jvm/`, `/usr/java/`, `/opt/java/`, `~/.sdkman/candidates/java/`, `~/.jdks/`
- Generates shell configuration for seamless switching

## Installation

### One-click install

```bash
git clone <repo-url>
cd java-version-switch
./install.sh
```

This will automatically:
- Install Rust toolchain if not present
- Build the project
- Copy `jvs` binary to `/usr/local/bin/`
- Configure your shell (`~/.zshrc` or `~/.bashrc`)
- Add a shell function wrapper for auto-sourcing

After installation, reload your shell:

```bash
source ~/.zshrc
```

## Usage

### List all installed Java versions

```bash
jvs list
```

Output example:
```
Installed Java versions:
  * 17.0.14  (Microsoft OpenJDK)  ~/Library/Java/JavaVirtualMachines/ms-17.0.14
    17.0.4.1 (Oracle Java SE)     /Library/Java/JavaVirtualMachines/jdk-17.0.4.1.jdk
    11.0.20  (Oracle Java SE)     /Library/Java/JavaVirtualMachines/jdk-11.jdk
    1.8.0_341 (Oracle Java SE)    /Library/Java/JavaVirtualMachines/jdk1.8.0_341.jdk
```

`*` indicates the currently active version.

### Show current Java version

```bash
jvs current
```

### Switch Java version

```bash
jvs use 11
```

This matches the version prefix, so `jvs use 11` will match `11.0.20`, `jvs use 1.8` will match `1.8.0_341`, etc.

The shell function wrapper automatically sources `~/.jvs/env` after switching, so the change takes effect immediately in your current shell.

## How It Works

1. **Detection**: Scans standard JDK directories for installed JDKs
   - macOS: `/Library/Java/JavaVirtualMachines/`, `~/Library/Java/JavaVirtualMachines/`
   - Linux: `/usr/lib/jvm/`, `/usr/java/`, `/opt/java/`, `~/.sdkman/candidates/java/`, `~/.jdks/`
2. **Identification**: Reads `release` file in each JDK home to extract version and vendor info
3. **PATH Cleanup**: Removes existing Java paths from PATH to prevent conflicts
4. **Switching**: Writes the selected JAVA_HOME to `~/.jvs/env` as an export statement
5. **Persistence**: Saves the selection to `~/.jvs/config.json`

## Configuration

Config file location: `~/.jvs/config.json`

```json
{
  "current": "/Library/Java/JavaVirtualMachines/jdk-11.jdk/Contents/Home"
}
```

Shell env file: `~/.jvs/env`

```bash
export JAVA_HOME="/Library/Java/JavaVirtualMachines/jdk-11.jdk/Contents/Home"
export PATH="$(echo "$PATH" | tr ':' '\n' | grep -v '/JavaVirtualMachines/' | tr '\n' ':' | sed 's/:$//')"
export PATH="$JAVA_HOME/bin:$PATH"
```

## Uninstall

```bash
./uninstall.sh
```

This will remove:
- The `jvs` binary from `/usr/local/bin/`
- The `~/.jvs/` directory
- Shell configuration entries from `~/.zshrc` or `~/.bashrc`

## Requirements

- macOS or Linux
- One or more JDK installations
- Rust 1.70+ (for building)

## License

MIT
