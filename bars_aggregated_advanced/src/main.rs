use clap::Parser;
use clap::ValueEnum;
use dotenv::dotenv;
use log::error;
use log::info;
use serde::Deserialize;
use std::env;
use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedBars {
    pub ticker: String,
    pub query_count: u8,
    pub results_count: u8,
    pub adjusted: bool,
    pub results: Vec<Bar>,
}

#[derive(Deserialize, Debug)]
pub struct Bar {
    pub v: f64,
    pub vw: f64,
    pub o: f64,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub t: i64,
    pub n: i64,
}

#[derive(ValueEnum, Clone, Deserialize, Debug)]
#[clap(rename_all = "kebab_case")]
pub enum Timespan {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl fmt::Display for Timespan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(ValueEnum, Clone, Deserialize, Debug)]
#[clap(rename_all = "kebab_case")]
pub enum Sort {
    Asc,
    Desc,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Ticker symbol
    #[arg(short, long)]
    symbol: String,

    /// Multiplier
    #[arg(short, long, default_value_t = 1)]
    multiplier: u8,

    /// Timespan
    #[arg(short, long)]
    timespan: Timespan,

    /// From
    #[arg(short, long)]
    from: String,

    /// To
    #[arg(long)]
    to: String,

    /// Adjusted
    #[arg(short, long, default_value_t = true)]
    adjusted: bool,

    /// Sort
    #[arg(long)]
    sort: Sort,
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Parse clap args
    let args = Args::parse();

    let symbol = args.symbol;
    let multiplier = args.multiplier;
    let timespan = args.timespan.to_string().to_lowercase();
    let from = args.from;
    let to = args.to;
    let adjusted = args.adjusted;
    let sort = args.sort.to_string().to_lowercase();

    // Start the logger
    env_logger::init();

    let token = "Bearer ".to_owned() + &env::var("POLYGON_API_KEY").unwrap();

    let url = format!("https://api.polygon.io/v2/aggs/ticker/{symbol}/range/{multiplier}/{timespan}/{from}/{to}?adjusted={adjusted}&sort={sort}");

    info!("{url}");

    let url_with_token = format!("https://api.polygon.io/v2/aggs/ticker/{symbol}/range/{multiplier}/{timespan}/{from}/{to}?adjusted={adjusted}&sort={sort}&apiKey={}", env::var("POLYGON_API_KEY").unwrap());

    let response = reqwest::get(url_with_token)
        .await
        .unwrap()
        .json::<AggregatedBars>()
        .await;

    match response {
        Ok(aggregated_bars) => {
            info!("Aggregated Bars with auth token in the url:");

            for bar in aggregated_bars.results {
                info!("{:?}", bar);
            }
        }
        Err(e) => {
            error!("API response cannot be parsed! {}", e)
        }
    };

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("Authorization", token)
        .send()
        .await
        .unwrap()
        .json::<AggregatedBars>()
        .await;

    match response {
        Ok(aggregated_bars) => {
            info!("Aggregated Bars with auth token in the header:");

            for bar in aggregated_bars.results {
                info!("{:?}", bar);
            }
        }
        Err(e) => {
            error!("API response cannot be parsed! {}", e)
        }
    };
}
