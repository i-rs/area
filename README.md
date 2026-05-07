# Area CLI

中国·国家地名信息库 CLI 查询工具

## 功能特性

- 地名搜索 - 按名称搜索地名
- 地名详情 - 获取地名详细信息
- 同名区划统计 - 统计重名行政区划
- 同名类别统计 - 按类别统计重名
- 同音区划统计 - 同音字区划统计
- 同音类别统计 - 同音字类别统计

## 安装

```bash
cargo build --release
```

或直接运行开发版本：

```bash
cargo run -- [OPTIONS] <COMMAND>
```

## 快速开始

### 搜索地名

```bash
# 搜索"故宫"
cargo run -- search 故宫

# 限定省份（北京）
cargo run -- search 故宫 -p 11

# 指定返回数量
cargo run -- search 故宫 -s 20
```

### 查看地名详情

```bash
cargo run -- detail <id>
```

### 输出格式

支持 JSON（默认）和 Table 两种输出格式：

```bash
# JSON 输出（默认）
cargo run -- search 故宫

# Table 输出
cargo run -- -o table search 故宫
```

## 全局选项

| 选项 | 说明 |
|------|------|
| `-o, --output <FORMAT>` | 输出格式: `json` 或 `table`（默认: json） |
| `--no-cache` | 跳过缓存，强制请求 API |
| `-h, --help` | 显示帮助信息 |
| `-V, --version` | 显示版本信息 |

## 数据缓存

本工具使用 SQLite 本地缓存查询结果，缓存有效期为 24 小时。

- 缓存路径: `~/.cache/.area_cache.db`
- 使用 `--no-cache` 可跳过缓存

## 项目结构

```
area/
├── src/
│   └── main.rs          # 主程序
├── Cargo.toml           # 依赖配置
├── docs/
│   ├── usage.md         # 详细使用文档
│   └── test.md         # 测试文档
└── README.md
```

## 依赖

- [clap](https://github.com/clap-rs/clap) - CLI 参数解析
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP 客户端
- [serde](https://github.com/serde-rs/serde) - 序列化/反序列化
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite 缓存
- [tokio](https://github.com/tokio-rs/tokio) - 异步运行时
- [indicatif](https://github.com/console-rs/indicatif) - 进度条

## 数据来源

本工具使用民政部提供的 [中国·国家地名信息库](https://dmfw.mca.gov.cn) API 接口。

坐标系: 2000国家大地坐标系（CGCS2000）
