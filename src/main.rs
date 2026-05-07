use clap::{Parser, Subcommand};
use console::Term;
use indicatif::ProgressBar;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

const BASE_URL: &str = "https://dmfw.mca.gov.cn";
const DB_PATH: &str = ".area_cache.db";

#[derive(Parser)]
#[command(name = "area")]
#[command(version = "0.1.0")]
#[command(about = "中国·国家地名信息库 CLI 查询工具", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "json")]
    output: OutputFormat,

    #[arg(long)]
    no_cache: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Table,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "地名搜索")]
    Search {
        #[arg(help = "地名关键词")]
        name: String,

        #[arg(short, long, help = "省份编码，如: 11表示北京")]
        province: Option<String>,

        #[arg(short, long, default_value = "10")]
        size: Option<usize>,
    },
    #[command(about = "地名详情获取")]
    Detail {
        #[arg(help = "地名ID")]
        id: String,
    },
    #[command(about = "同名区划统计")]
    SameNameDistrict {
        #[arg(help = "地名名称")]
        name: String,

        #[arg(short, long)]
        district: Option<String>,
    },
    #[command(about = "同名类别统计")]
    SameNameType {
        #[arg(help = "地名名称")]
        name: String,

        #[arg(short, long)]
        place_type: Option<String>,
    },
    #[command(about = "同音区划统计")]
    HomophoneDistrict {
        #[arg(help = "地名名称")]
        name: String,

        #[arg(short, long)]
        district: Option<String>,
    },
    #[command(about = "同音类别统计")]
    HomophoneType {
        #[arg(help = "地名名称")]
        name: String,

        #[arg(short, long)]
        place_type: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SearchRequest {
    page: usize,
    size: usize,
    code: Option<String>,
    #[serde(rename = "stName")]
    stname: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResponse {
    total: usize,
    records: Vec<PlaceRecord>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PlaceRecord {
    id: Option<String>,
    #[serde(rename = "standard_name")]
    standard_name: Option<String>,
    #[serde(rename = "place_type")]
    place_type: Option<String>,
    #[serde(rename = "place_type_code")]
    place_type_code: Option<String>,
    #[serde(rename = "place_code")]
    place_code: Option<String>,
    #[serde(rename = "province")]
    province: Option<String>,
    #[serde(rename = "province_name")]
    province_name: Option<String>,
    #[serde(rename = "city")]
    city: Option<String>,
    #[serde(rename = "city_name")]
    city_name: Option<String>,
    #[serde(rename = "area")]
    area: Option<String>,
    #[serde(rename = "area_name")]
    area_name: Option<String>,
    #[serde(rename = "roman_alphabet_spelling")]
    roman_alphabet_spelling: Option<String>,
    #[serde(rename = "ethnic_minorities_writing")]
    ethnic_minorities_writing: Option<String>,
    #[serde(rename = "gdm")]
    gdm: Option<serde_json::Value>,
    #[serde(rename = "pdm")]
    pdm: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DetailRequest {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DetailResponse {
    #[serde(rename = "standard_name")]
    standard_name: Option<String>,
    #[serde(rename = "place_type")]
    place_type: Option<String>,
    #[serde(rename = "place_type_code")]
    place_type_code: Option<String>,
    #[serde(rename = "place_code")]
    place_code: Option<String>,
    #[serde(rename = "province_name")]
    province_name: Option<String>,
    #[serde(rename = "city_name")]
    city_name: Option<String>,
    #[serde(rename = "area_name")]
    area_name: Option<String>,
    #[serde(rename = "lon")]
    lon: Option<f64>,
    #[serde(rename = "lat")]
    lat: Option<f64>,
    #[serde(rename = "source")]
    source: Option<String>,
    #[serde(rename = "update_time")]
    update_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StatsResponse {
    total: usize,
    records: Vec<StatsRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StatsRecord {
    #[serde(rename = "place_type")]
    place_type: Option<String>,
    num: usize,
    #[serde(rename = "area_name")]
    area_name: Option<String>,
}

struct Cache {
    conn: Connection,
}

impl Cache {
    fn new() -> Result<Self, rusqlite::Error> {
        let cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
        let db_path = cache_dir.join(DB_PATH);
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS cache (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    fn get(&self, key: &str) -> Option<String> {
        let cache_ttl = 3600 * 24;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let row: Result<(String, i64), _> = self.conn.query_row(
            "SELECT value, timestamp FROM cache WHERE key = ?",
            [key],
            |row| Ok((row.get(0)?, row.get(1)?)),
        );

        match row {
            Ok((value, timestamp)) if now - timestamp < cache_ttl => Some(value),
            _ => None,
        }
    }

    fn set(&self, key: &str, value: &str) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let _ = self.conn.execute(
            "INSERT OR REPLACE INTO cache (key, value, timestamp) VALUES (?, ?, ?)",
            params![key, value, now],
        );
    }
}

impl Drop for Cache {
    fn drop(&mut self) {}
}

fn get_cache() -> Option<Mutex<Cache>> {
    Cache::new().ok().map(|c| Mutex::new(c))
}

async fn api_get<T: for<'de> Deserialize<'de>>(path: &str, cache: &Option<Mutex<Cache>>) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}{}", BASE_URL, path);

    if let Some(cache) = cache {
        let cache_key = path;
        if let Ok(guard) = cache.lock() {
            if let Some(cached) = guard.get(cache_key) {
                return serde_json::from_str(&cached).map_err(Into::into);
            }
        }
    }

    let pb = ProgressBar::new_spinner();
    pb.set_message("正在查询...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;
    let text = resp.text().await?;

    pb.finish_with_message("查询完成");

    if let Some(cache) = cache {
        let cache_key = path;
        if let Ok(guard) = cache.lock() {
            guard.set(cache_key, &text);
        }
    }

    serde_json::from_str(&text).map_err(Into::into)
}

async fn api_post<T: for<'de> Deserialize<'de>, R: Serialize>(path: &str, body: &R, cache: &Option<Mutex<Cache>>) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}{}", BASE_URL, path);

    let body_str = serde_urlencoded::to_string(body).unwrap_or_default();

    if let Some(cache) = cache {
        let cache_key = format!("{}:{}", path, body_str);
        if let Ok(guard) = cache.lock() {
            if let Some(cached) = guard.get(&cache_key) {
                return serde_json::from_str(&cached).map_err(Into::into);
            }
        }
    }

    let pb = ProgressBar::new_spinner();
    pb.set_message("正在查询...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let client = reqwest::Client::new();
    let resp = client.post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .body(body_str.clone())
        .send().await?;
    let text = resp.text().await?;

    pb.finish_with_message("查询完成");

    if let Some(cache) = cache {
        let cache_key = format!("{}:{}", path, body_str);
        if let Ok(guard) = cache.lock() {
            guard.set(&cache_key, &text);
        }
    }

    serde_json::from_str(&text).map_err(Into::into)
}

fn print_json<T: Serialize>(data: &T) {
    let json = serde_json::to_string_pretty(data).unwrap();
    println!("{}", json);
}

fn print_table(records: &[PlaceRecord]) {
    let term = Term::stdout();
    let width = term.size().1 as usize;
    let col_width = (width / 4).max(20);

    let headers = ["标准名称", "类型", "省份", "城市"];
    let separator = "-".repeat(width);

    println!("{}", headers.iter()
        .map(|h| format!("{:^width$}", h, width = col_width))
        .collect::<Vec<_>>()
        .join(" | "));
    println!("{}", separator);

    for record in records {
        println!("{} | {} | {} | {}",
            record.standard_name.as_deref().unwrap_or("-"),
            record.place_type.as_deref().unwrap_or("-"),
            record.province_name.as_deref().unwrap_or("-"),
            record.city_name.as_deref().unwrap_or("-")
        );
    }
}

fn print_stats_table(records: &[StatsRecord]) {
    let term = Term::stdout();
    let width = term.size().1 as usize;
    let col_width = (width / 3).max(15);

    let headers = ["类别/地区", "数量"];
    let separator = "-".repeat(width);

    println!("{}", headers.iter()
        .map(|h| format!("{:^width$}", h, width = col_width))
        .collect::<Vec<_>>()
        .join(" | "));
    println!("{}", separator);

    for record in records {
        let name = record.place_type.as_deref()
            .or(record.area_name.as_deref())
            .unwrap_or("-");
        println!("{} | {}",
            format!("{:^width$}", name, width = col_width),
            record.num
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    let cache = if cli.no_cache { None } else { get_cache() };

    match &cli.command {
        Commands::Search { name, province, size } => {
            let req = SearchRequest {
                page: 1,
                size: size.unwrap_or(10),
                code: province.clone(),
                stname: name.clone(),
            };
            let resp: SearchResponse = api_post("/stname/listPub", &req, &cache).await?;

            match cli.output {
                OutputFormat::Json => print_json(&resp),
                OutputFormat::Table => print_table(&resp.records),
            }
        }

        Commands::Detail { id } => {
            let req = DetailRequest { id: id.clone() };
            let resp: DetailResponse = api_post("/stname/detailsPub", &req, &cache).await?;

            match cli.output {
                OutputFormat::Json => print_json(&resp),
                OutputFormat::Table => {
                    println!("标准名称: {}", resp.standard_name.unwrap_or_default());
                    println!("类    型: {}", resp.place_type.unwrap_or_default());
                    println!("省    份: {}", resp.province_name.unwrap_or_default());
                    println!("城    市: {}", resp.city_name.unwrap_or_default());
                    println!("区    县: {}", resp.area_name.unwrap_or_default());
                    if let (Some(lon), Some(lat)) = (resp.lon, resp.lat) {
                        println!("坐    标: {}, {}", lon, lat);
                    }
                    println!("数据来源: {}", resp.source.unwrap_or_default());
                    println!("更新时间: {}", resp.update_time.unwrap_or_default());
                }
            }
        }

        Commands::SameNameDistrict { name, district } => {
            let mut params = vec![("stname", name.as_str())];
            if let Some(d) = district {
                params.push(("area", d));
            }
            let query = params.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            let resp: StatsResponse = api_get(&format!("/stname/sameNameDistrict?{}", query), &cache).await?;

            match cli.output {
                OutputFormat::Json => print_json(&resp),
                OutputFormat::Table => print_stats_table(&resp.records),
            }
        }

        Commands::SameNameType { name, place_type } => {
            let mut params = vec![("stname", name.as_str())];
            if let Some(pt) = place_type {
                params.push(("placeType", pt));
            }
            let query = params.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            let resp: StatsResponse = api_get(&format!("/stname/sameNameType?{}", query), &cache).await?;

            match cli.output {
                OutputFormat::Json => print_json(&resp),
                OutputFormat::Table => print_stats_table(&resp.records),
            }
        }

        Commands::HomophoneDistrict { name, district } => {
            let mut params = vec![("stname", name.as_str())];
            if let Some(d) = district {
                params.push(("area", d));
            }
            let query = params.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            let resp: StatsResponse = api_get(&format!("/stname/sameNamePhoneticDistrict?{}", query), &cache).await?;

            match cli.output {
                OutputFormat::Json => print_json(&resp),
                OutputFormat::Table => print_stats_table(&resp.records),
            }
        }

        Commands::HomophoneType { name, place_type } => {
            let mut params = vec![("stname", name.as_str())];
            if let Some(pt) = place_type {
                params.push(("placeType", pt));
            }
            let query = params.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            let resp: StatsResponse = api_get(&format!("/stname/sameNamePhoneticType?{}", query), &cache).await?;

            match cli.output {
                OutputFormat::Json => print_json(&resp),
                OutputFormat::Table => print_stats_table(&resp.records),
            }
        }
    }

    Ok(())
}
