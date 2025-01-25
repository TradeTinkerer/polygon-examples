use dotenv::dotenv;
use log::error;
use log::info;
use serde::Deserialize;
use std::env;

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

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Start the logger
    env_logger::init();

    let token = "Bearer ".to_owned() + &env::var("POLYGON_API_KEY").unwrap();

    let url = "https://api.polygon.io/v2/aggs/ticker/AAPL/range/1/day/2023-01-09/2023-02-10?adjusted=true&sort=asc";

    let url_with_token = format!("https://api.polygon.io/v2/aggs/ticker/AAPL/range/1/day/2023-01-09/2023-02-10?adjusted=true&sort=asc&apiKey={}", env::var("POLYGON_API_KEY").unwrap());

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
            error!("Orders API response cannot be parsed! {}", e)
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
            error!("Orders API response cannot be parsed! {}", e)
        }
    };
}
