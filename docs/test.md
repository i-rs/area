# 测试文档

## 构建测试

### 开发模式构建

```bash
cargo build
```

### 发布模式构建

```bash
cargo build --release
```

### 检查代码

```bash
# 检查代码语法和类型
cargo check

# 运行 clippy linter
cargo clippy
```

## 功能测试

### 1. 帮助信息测试

```bash
cargo run -- --help
```

**预期输出：**
```
中国·国家地名信息库 CLI 查询工具

Usage: area [OPTIONS] <COMMAND>

Commands:
  search              地名搜索
  detail              地名详情获取
  same-name-district  同名区划统计
  same-name-type      同名类别统计
  homophone-district  同音区划统计
  homophone-type      同音类别统计
  help                Print this message or the help of the given subcommand(s)

Options:
  -o, --output <OUTPUT>  [default: json] [possible values: json, table]
      --no-cache
  -h, --help             Print help
  -V, --version          Print version
```

### 2. 子命令帮助测试

```bash
cargo run -- search --help
```

**预期输出：**
```
地名搜索

Usage: area search [OPTIONS] <NAME>

Arguments:
  <NAME>  地名关键词

Options:
  -p, --province <PROVINCE>  省份编码，如: 11表示北京
  -s, --size <SIZE>          [default: 10]
  -h, --help                 Print help
```

### 3. 地名搜索测试（JSON格式）

```bash
cargo run -- search 故宫
```

**预期行为：**
- 显示进度条 "正在查询..."
- 查询完成后显示 JSON 格式结果
- 返回包含 `total` 和 `records` 字段的 JSON

**验证点：**
- [ ] 返回状态为成功
- [ ] JSON 包含 `total` 字段（数字类型）
- [ ] JSON 包含 `records` 字段（数组类型）
- [ ] 每条记录包含 `id`、`standard_name`、`place_type` 等字段

### 4. 地名搜索测试（Table格式）

```bash
cargo run -- -o table search 故宫
```

**预期输出：**
```
        标准名称         |          类型          |          省份          |          城市
----------------------------------------------------------------------
故宫 | 农村居民点 | 云南省 | 玉溪市
明故宫路 | 主干路 | 江苏省 | 南京市
...
```

**验证点：**
- [ ] 表头正确显示：标准名称、类型、省份、城市
- [ ] 数据行正确对齐
- [ ] 中文显示正常

### 5. 省份筛选测试

```bash
cargo run -- search 故宫 -p 11
```

**验证点：**
- [ ] 只返回北京市的地名
- [ ] 其他省份的数据被过滤

### 6. 自定义返回数量测试

```bash
cargo run -- search 故宫 -s 5
```

**验证点：**
- [ ] 返回数量为 5 条或更少

### 7. 地名详情测试

```bash
# 先从搜索结果获取一个 ID
cargo run -- search 故宫
# 假设获取到 ID: ec859ba1bd23efb7b09f6a5b8cf9a99d

cargo run -- detail ec859ba1bd23efb7b09f6a5b8cf9a99d
```

**验证点：**
- [ ] 返回该 ID 对应的详细信息
- [ ] JSON 包含 `standard_name`、`place_type`、`province_name` 等字段

### 8. 缓存测试

```bash
# 第一次请求（无缓存）
cargo run -- --no-cache search 故宫

# 第二次请求（有缓存）
cargo run -- search 故宫
```

**验证点：**
- [ ] 第二次请求速度更快
- [ ] 结果与第一次一致

### 9. 缓存跳过测试

```bash
cargo run -- --no-cache search 故宫
```

**验证点：**
- [ ] 跳过缓存，直接请求 API

## API 集成测试

### 测试环境要求

- 网络连接正常
- 能够访问 `https://dmfw.mca.gov.cn`

### 测试用例矩阵

| 测试用例 | 输入 | 预期结果 |
|---------|------|---------|
| 基本搜索 | `search 故宫` | 返回包含"故宫"的地名列表 |
| 空搜索 | `search xyzabc123` | 返回空列表或错误提示 |
| 省份筛选 | `search 故宫 -p 11` | 只返回北京的数据 |
| 数量限制 | `search 故宫 -s 1` | 返回 1 条数据 |
| 详情查询 | `detail <valid_id>` | 返回该地点详细信息 |
| 无效ID | `detail invalid-id` | 返回错误或空结果 |

## 性能测试

### 响应时间基准

| 操作 | 预期最大时间 |
|------|-------------|
| 搜索（无缓存） | < 5 秒 |
| 搜索（有缓存） | < 1 秒 |
| 详情查询 | < 3 秒 |

### 测试命令

```bash
# 测量搜索时间
time cargo run -- search 故宫

# 测量详情查询时间
time cargo run -- detail ec859ba1bd23efb7b09f6a5b8cf9a99d
```

## 边界条件测试

### 1. 超长搜索词

```bash
cargo run -- search $(python3 -c "print('测' * 100)")
```

### 2. 特殊字符

```bash
cargo run -- search "故宫*"
cargo run -- search "故宫'"
```

### 3. 空搜索词

```bash
cargo run -- search ""
```

## 回归测试清单

每次代码更新后，运行以下测试用例确保功能正常：

- [ ] `--help` 显示帮助信息
- [ ] `search 故宫` 返回结果
- [ ] `search 故宫 -o table` 显示表格
- [ ] `search 故宫 -p 11` 过滤省份
- [ ] `detail <id>` 显示详情
- [ ] `--no-cache` 跳过缓存
- [ ] 构建无警告 (`cargo build`)
