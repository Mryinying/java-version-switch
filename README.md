# JVS - Java Version Switch

轻量级 Java 版本管理 CLI 工具，使用 Rust 编写，支持 macOS 和 Linux。

## 功能特性

- 列出所有已安装的 Java 版本
- 显示当前使用的 Java 版本
- 通过前缀匹配快速切换 Java 版本
- 自动清理 PATH，防止版本冲突
- Shell 函数包装器，切换后即时生效，无需手动 source
- 通过系统包管理器（brew/apt/yum）安装 JDK
- 通过系统包管理器卸载 JDK
- 支持标准 JDK 安装路径：
  - macOS: `/Library/Java/JavaVirtualMachines/`、`~/Library/Java/JavaVirtualMachines/`
  - Linux: `/usr/lib/jvm/`、`/usr/java/`、`/opt/java/`、`~/.sdkman/candidates/java/`、`~/.jdks/`

## 安装

```bash
git clone <repo-url>
cd java-version-switch
./install.sh
```

安装脚本会自动完成：
- 检测并安装 Rust 工具链（如未安装）
- 编译项目
- 将 `jvs` 可执行文件复制到 `/usr/local/bin/`
- 在 shell 配置文件（`~/.zshrc` 或 `~/.bashrc`）中添加自动加载配置和函数包装器

安装完成后，重新加载 shell：

```bash
source ~/.zshrc
```

## 使用方法

### 列出已安装的 Java 版本

```bash
jvs list
```

输出示例：
```
Installed Java versions:
  * 17.0.14  (Microsoft)           ~/Library/Java/JavaVirtualMachines/ms-17.0.14
    17.0.4.1 (Oracle Corporation)  /Library/Java/JavaVirtualMachines/jdk-17.0.4.1.jdk
    11.0.20  (Oracle Corporation)  /Library/Java/JavaVirtualMachines/jdk-11.jdk
    1.8.0_341 (Unknown)            /Library/Java/JavaVirtualMachines/jdk1.8.0_341.jdk
```

`*` 标记表示当前正在使用的版本。

### 查看当前 Java 版本

```bash
jvs current
```

### 切换 Java 版本

```bash
jvs use 11
```

支持前缀匹配：`jvs use 11` 匹配 `11.0.20`，`jvs use 1.8` 匹配 `1.8.0_341`。

通过 `install.sh` 安装后，切换会立即在当前 shell 中生效，无需手动操作。

### 安装 JDK

```bash
jvs install <主版本号>
```

通过系统包管理器安装 JDK：

| 平台 | 包管理器 | JDK 来源 |
|------|---------|----------|
| macOS | brew | Eclipse Temurin (`temurin@{version}`) |
| Linux | apt | OpenJDK (`openjdk-{version}-jdk`) |
| Linux | yum | OpenJDK (`java-{version}-openjdk-devel`) |

示例：
```bash
jvs install 21    # 安装 JDK 21
jvs list          # 查看安装结果
jvs use 21        # 切换到新版本
```

### 卸载 JDK

```bash
jvs remove <主版本号>
```

示例：
```bash
jvs remove 21
```

## 工作原理

1. **检测**：扫描标准 JDK 安装目录
   - macOS: `/Library/Java/JavaVirtualMachines/`、`~/Library/Java/JavaVirtualMachines/`
   - Linux: `/usr/lib/jvm/`、`/usr/java/`、`/opt/java/`、`~/.sdkman/candidates/java/`、`~/.jdks/`
2. **识别**：读取每个 JDK 的 `release` 文件，提取版本和厂商信息
3. **PATH 清理**：切换时移除 PATH 中残留的旧 Java 路径，防止冲突
4. **切换**：将选中的 JAVA_HOME 写入 `~/.jvs/env`
5. **持久化**：将选择保存到 `~/.jvs/config.json`

## 配置文件

配置文件位置：`~/.jvs/config.json`

```json
{
  "current": "/Library/Java/JavaVirtualMachines/jdk-11.jdk/Contents/Home"
}
```

Shell 环境文件：`~/.jvs/env`

```bash
export JAVA_HOME="/Library/Java/JavaVirtualMachines/jdk-11.jdk/Contents/Home"
export PATH="$(echo "$PATH" | tr ':' '\n' | grep -v '/JavaVirtualMachines/' | tr '\n' ':' | sed 's/:$//')"
export PATH="$JAVA_HOME/bin:$PATH"
```

## 卸载工具

```bash
./uninstall.sh
```

将移除：
- `/usr/local/bin/jvs` 可执行文件
- `~/.jvs/` 配置目录
- shell 配置文件中的 JVS 配置块

## 系统要求

- macOS 或 Linux
- 至少安装一个 JDK
- Rust 1.70+（用于编译）

## 许可证

MIT
