# Keylogger-rs

一个使用Rust编写的跨平台键盘记录器，支持普通模式和Windows服务模式运行。

## 功能特点

- 跨平台支持（Windows/Linux/macOS）
- 支持以Windows服务方式运行（仅Windows系统）
- 可配置的日志保存路径和保存间隔
- 键盘按键统计和记录
- 支持通过配置文件自定义设置

## 系统要求

- Rust 1.70.0 或更高版本
- Windows 10/11（如需使用Windows服务模式）

## 安装

```bash
# 克隆项目
git clone https://github.com/seaung/keylogger-rs.git
cd keylogger-rs

# 编译项目
cargo build --release
```

## 配置

在程序运行目录下创建`config.toml`文件，可以自定义以下配置项：

```toml
# 按键记录保存路径
log_path = "keypress.json"

# 保存间隔（秒）
save_interval = 1

# 是否以Windows服务模式运行（仅Windows系统有效）
run_as_service = false
```

## 使用方法

### 普通模式

直接运行编译后的可执行文件即可：

```bash
./target/release/keylogger-rs
```

程序会在后台运行，并将按键记录保存到配置文件中指定的路径。

### Windows服务模式（仅Windows系统）

1. 以管理员权限运行命令提示符

2. 安装服务：
```cmd
sc create KeyloggerService binPath= "完整路径\keylogger-rs.exe"
```

3. 启动服务：
```cmd
sc start KeyloggerService
```

4. 停止服务：
```cmd
sc stop KeyloggerService
```

5. 删除服务：
```cmd
sc delete KeyloggerService
```

## 日志格式

按键记录保存为JSON格式，包含按键名称和按键次数：

```json
{"key":"A", "count": 10}
{"key":"Space", "count": 5}
```

## 注意事项

- 在Windows系统中使用服务模式时需要管理员权限
- 建议定期检查和清理日志文件，避免占用过多磁盘空间
- 请遵守相关法律法规，不要将此工具用于非法用途

## 许可证

MIT License