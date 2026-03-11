# JVS 开发文档

## 项目概述

JVS (Java Version Switch) 是一个用 Rust 编写的 macOS / Linux Java 版本管理命令行工具。

## 技术栈

| 技术 | 版本 | 用途 |
|------|------|------|
| Rust | 1.70+ | 主语言 |
| clap | 4.x | CLI 参数解析 (derive 模式) |
| colored | 2.x | 终端彩色输出 |
| serde/serde_json | 1.x | 配置文件序列化 |
| dirs | 5.x | 跨平台目录路径 |

## 项目结构

```
java-version-switch/
├── Cargo.toml          # 项目依赖配置
├── Cargo.lock          # 依赖锁定文件
├── LICENSE             # MIT 许可证
├── README.md           # 英文说明文档
├── install.sh          # 一键安装脚本
├── uninstall.sh        # 一键卸载脚本
├── docs/
│   ├── DEVELOPMENT.md  # 开发文档 (本文件)
│   └── 使用文档.md      # 中文操作文档
└── src/
    └── main.rs         # 程序入口与全部逻辑
```

## 架构设计

### 模块划分

程序采用单文件结构，逻辑上分为以下模块：

1. **CLI 定义** (`Cli` / `Commands` enum) - 使用 clap derive 宏定义命令行接口
2. **Java 检测** (`JavaVersion` struct + `detect_java_versions()`) - 扫描 macOS / Linux JDK 安装目录
3. **版本切换** (`cmd_use()`) - 写入 shell 环境文件和持久化配置
4. **配置管理** (`JvsConfig` struct) - JSON 配置文件读写

### 核心数据结构

```rust
struct JavaVersion {
    version: String,     // e.g. "17.0.14"
    vendor: String,      // e.g. "Microsoft"
    home: PathBuf,       // JAVA_HOME path
}

struct JvsConfig {
    current: Option<String>,  // 当前选中的 JAVA_HOME 路径
}
```

### Java 版本检测流程

```
扫描目录:
  macOS:
    /Library/Java/JavaVirtualMachines/*/Contents/Home
    ~/Library/Java/JavaVirtualMachines/*/Contents/Home
  Linux:
    /usr/lib/jvm/*, /usr/java/*, /opt/java/*
    ~/.sdkman/candidates/java/*, ~/.jdks/*
      ↓
读取每个 JDK 的 release 文件
      ↓
解析 JAVA_VERSION 和 IMPLEMENTOR 字段
      ↓
构建 JavaVersion 列表并按版本排序
```

### 版本切换流程

```
用户输入: jvs use <version_prefix>
      ↓
模糊匹配已安装版本 (前缀匹配)
      ↓
写入 ~/.jvs/env (export JAVA_HOME=... 和 PATH 清理)
      ↓
写入 ~/.jvs/config.json (持久化选择)
      ↓
shell 函数包装器自动 source ~/.jvs/env，即时生效
```

## 构建与测试

### 开发构建

```bash
cargo build
```

### Release 构建

```bash
cargo build --release
```

### 运行

```bash
cargo run -- list
cargo run -- current
cargo run -- use 17
```

## 关键设计决策

1. **单文件结构**: 项目逻辑简单，单文件便于维护和理解
2. **前缀匹配**: `jvs use 11` 匹配 `11.0.20`，提升用户体验
3. **Shell env 文件**: 通过 `~/.jvs/env` 导出环境变量，兼容 bash/zsh
4. **release 文件解析**: 直接读取 JDK 的 `release` 文件获取版本信息，比执行 `java -version` 更快更可靠
5. **不依赖 /usr/libexec/java_home**: 直接扫描文件系统，更透明可控
6. **跨平台**: 通过 `cfg!(target_os)` 区分 macOS 和 Linux 扫描路径

## 扩展方向

- 支持项目级 `.java-version` 文件
- 添加 `jvs default` 命令设置默认版本
- Homebrew formula 发布
动下载安装 JDK (类似 sdkman)
- 支持项目级 `.java-version` 文件
- 添加 `jvs default` 命令设置默认版本
- Homebrew formula 发布
