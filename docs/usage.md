# 使用文档

## 命令概述

```
area [OPTIONS] <COMMAND>
```

## 可用命令

### 1. search - 地名搜索

按地名名称搜索相关地点。

```bash
area search <NAME> [OPTIONS]
```

**参数：**

| 参数 | 说明 |
|------|------|
| `<NAME>` | 地名关键词（必填） |

**选项：**

| 选项 | 说明 |
|------|------|
| `-p, --province <CODE>` | 省份编码，如: `11` 表示北京 |
| `-s, --size <SIZE>` | 返回数量（默认: 10） |
| `-h, --help` | 显示帮助 |

**示例：**

```bash
# 基本搜索
area search 故宫

# 限定北京市（省份编码 11）
area search 故宫 -p 11

# 指定返回 20 条结果
area search 故宫 -s 20

# 限定省份并指定数量
area search 故宫 -p 11 -s 20
```

**输出示例（JSON）：**

```json
{
  "total": 37,
  "records": [
    {
      "id": "ec859ba1bd23efb7b09f6a5b8cf9a99d",
      "standard_name": "故宫",
      "place_type": "农村居民点",
      "place_type_code": "22200",
      "place_code": "53042710222200000188",
      "province_name": "云南省",
      "city_name": "玉溪市",
      "area_name": "新平彝族傣族自治县",
      "roman_alphabet_spelling": "Gùgōng",
      "gdm": {
        "type": "multipoint",
        "coordinates": [[101.7342088, 23.8174111]]
      }
    }
  ]
}
```

**输出示例（Table）：**

```
        标准名称         |          类型          |          省份          |          城市
----------------------------------------------------------------------
故宫 | 农村居民点 | 云南省 | 玉溪市
明故宫路 | 主干路 | 江苏省 | 南京市
```

### 2. detail - 地名详情获取

根据 ID 获取地名的详细信息。

```bash
area detail <ID>
```

**参数：**

| 参数 | 说明 |
|------|------|
| `<ID>` | 地名 ID（从 search 命令获取） |

**示例：**

```bash
area detail ec859ba1bd23efb7b09f6a5b8cf9a99d
```

**输出示例：**

```json
{
  "standard_name": "故宫",
  "place_type": "农村居民点",
  "place_type_code": "22200",
  "place_code": "53042710222200000188",
  "province_name": "云南省",
  "city_name": "玉溪市",
  "area_name": "新平彝族傣族自治县",
  "lon": null,
  "lat": null,
  "source": null,
  "update_time": null
}
```

### 3. same-name-district - 同名区划统计

统计指定名称在不同行政区划的重名情况。

```bash
area same-name-district <NAME> [OPTIONS]
```

**参数：**

| 参数 | 说明 |
|------|------|
| `<NAME>` | 地名名称（必填） |

**选项：**

| 选项 | 说明 |
|------|------|
| `-d, --district <CODE>` | 区划编码 |
| `-h, --help` | 显示帮助 |

### 4. same-name-type - 同名类别统计

按地名类别统计重名情况。

```bash
area same-name-type <NAME> [OPTIONS]
```

**参数：**

| 参数 | 说明 |
|------|------|
| `<NAME>` | 地名名称（必填） |

**选项：**

| 选项 | 说明 |
|------|------|
| `-t, --place-type <TYPE>` | 地名类别 |
| `-h, --help` | 显示帮助 |

### 5. homophone-district - 同音区划统计

统计指定名称的同音字在不同行政区划的重名情况。

```bash
area homophone-district <NAME> [OPTIONS]
```

### 6. homophone-type - 同音类别统计

按地名类别统计同音字重名情况。

```bash
area homophone-type <NAME> [OPTIONS]
```

## 全局选项

### 输出格式控制

```bash
# JSON 格式（默认）
area search 故宫

# Table 格式
area -o table search 故宫
```

### 缓存控制

```bash
# 跳过缓存，强制请求最新数据
area --no-cache search 故宫
```

## 省份编码参考

| 省份 | 编码 |
|------|------|
| 北京市 | 11 |
| 天津市 | 12 |
| 河北省 | 13 |
| 山西省 | 14 |
| 内蒙古 | 15 |
| 辽宁省 | 21 |
| 吉林省 | 22 |
| 黑龙江省 | 23 |
| 上海市 | 31 |
| 江苏省 | 32 |
| 浙江省 | 33 |
| 安徽省 | 34 |
| 福建省 | 35 |
| 江西省 | 36 |
| 山东省 | 37 |
| 河南省 | 41 |
| 湖北省 | 42 |
| 湖南省 | 43 |
| 广东省 | 44 |
| 广西 | 45 |
| 海南省 | 46 |
| 重庆市 | 50 |
| 四川省 | 51 |
| 贵州省 | 52 |
| 云南省 | 53 |
| 西藏 | 54 |
| 陕西省 | 61 |
| 甘肃省 | 62 |
| 青海省 | 63 |
| 宁夏 | 64 |
| 新疆 | 65 |

## 常见问题

### Q: 搜索结果为空？

A: 可能原因：
1. 名称拼写有误
2. 该地名不在数据库中
3. 尝试使用更简短的关键词

### Q: 缓存数据过期？

A: 使用 `--no-cache` 选项跳过缓存，获取最新数据。

### Q: API 请求失败？

A: 检查网络连接，或使用 `--no-cache` 重新请求。
