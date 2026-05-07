---
name: "area-cli"
description: "中国地名信息库 CLI 工具。用于查询地名、获取行政区划、统计重名等。当用户需要查询地名、获取地址信息、搜索地点或需要地名数据时调用。"
---

# Area CLI - 中国地名信息查询工具

area-cli 是基于民政部中国·国家地名信息库 API 开发的命令行工具，提供地名搜索、详情查询、重名统计等功能。

## 项目位置

`/Users/mankong/volumes/code/i-rs/area`

## 快速命令参考

```bash
# 搜索地名
area search <关键词>

# 查看详情（需要ID）
area detail <id>

# 同名区划统计
area same-name-district <名称>

# 同名类别统计
area same-name-type <名称>

# 同音区划统计
area homophone-district <名称>

# 同音类别统计
area homophone-type <名称>
```

## 全局选项

| 选项 | 说明 |
|------|------|
| `-o, --output <json\|table>` | 输出格式，默认 json |
| `--no-cache` | 跳过缓存，强制请求 API |
| `-h, --help` | 显示帮助 |

## 常用示例

### 1. 搜索地名

```bash
# 基本搜索
area search 故宫

# 限定省份（北京代码11）
area search 故宫 -p 11

# 表格输出
area search 故宫 -o table

# 指定返回数量
area search 故宫 -s 20
```

### 2. 查看地名详情

```bash
# 先搜索获取ID
area search 故宫

# 用ID查看详情
area detail 27612aa5-1e2f-4e40-ac9f-a44a23625b61
```

### 3. 统计功能

```bash
# 统计"友谊"同名区划
area same-name-district 友谊

# 统计"广场"同名类别
area same-name-type 广场
```

## 项目结构

```
area/
├── src/main.rs           # 主程序代码
├── Cargo.toml            # 依赖配置
├── .github/workflows/    # CI/CD 配置
├── wix/                  # Windows MSI 配置
├── docs/                 # 文档
│   ├── usage.md         # 使用文档
│   ├── test.md          # 测试文档
│   └── promotion.md      # 推广文章
└── README.md
```

## 构建与发布

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 跨平台构建（需 cargo-dist）
cargo dist build

# 生成 CI 配置
cargo dist generate
```

## 数据来源

API: https://dmfw.mca.gov.cn
坐标系: CGCS2000 (2000国家大地坐标系)

## 依赖技术栈

- **clap** - CLI 参数解析
- **reqwest** - HTTP 客户端
- **serde** - JSON 序列化
- **rusqlite** - SQLite 缓存
- **tokio** - 异步运行时
- **cargo-dist** - 跨平台构建分发
