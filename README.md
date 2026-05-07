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

## CI/CD 配置

### 环境变量

| 变量名 | 说明 |
|--------|------|
| `NPM_TOKEN` | npm 发布令牌，有效期 **90 天**，需定期更新 |

### NPM_TOKEN 管理

1. **创建 Token**: 在 npm 账户设置中创建 "Automation" 或 "Publish" 类型的 token
2. **有效期**: 默认 90 天，过期后需要重新生成
3. **配置位置**: 在 GitHub 仓库的 `Settings > Secrets and variables > Actions` 中添加
4. **更新周期**: 建议每 80 天更新一次，避免发布失败

### 发布流程

```bash
# 创建版本 tag
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions 会自动完成：
- 构建各平台二进制包
- 生成 npm 包（包名: `@i-rs/area`）
- 发布到 GitHub Releases
- 发布到 npm 仓库
