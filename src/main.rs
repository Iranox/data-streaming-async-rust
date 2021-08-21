use chrono::prelude::*;
use std::io::{Error, ErrorKind};
use yahoo_finance_api as yahoo;

fn max(series: &[f64]) -> Option<f64> {
    if !series.is_empty() {
        return Some(series.iter().fold(f64::MIN, |pre, suc| pre.max(*suc)));
    }
    None
}

fn min(series: &[f64]) -> Option<f64> {
    if !series.is_empty() {
        return Some(series.iter().fold(f64::MIN, |pre, suc| pre.min(*suc)));
    }
    None
}

fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    if series.is_empty() {
        return None;
    }

    let (last_price, first_price) = (series.last().unwrap(), series.first().unwrap());
    let price_diff_absolute = last_price - first_price;
    let first = if *first_price == 0.0 { 1.0 } else { *first_price };
    let price_diff_r = price_diff_absolute / first;

    Some((price_diff_absolute, price_diff_r))
}

fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if series.is_empty() {
        return None;
    }
    Some(
        series
            .windows(n)
            .map(|w| w.iter().sum::<f64>() / w.len() as f64)
            .collect(),
    )
}

fn fetch_closing_data(
    symbol: &str,
    beginning: &DateTime<Utc>,
    end: &DateTime<Utc>,
) -> std::io::Result<Vec<f64>> {
    let provider = yahoo::YahooConnector::new();

    let response = provider
        .get_quote_history(symbol, *beginning, *end)
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut quotes = response
        .quotes()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    if !quotes.is_empty() {
        quotes.sort_by_cached_key(|k| k.timestamp);
        Ok(quotes.iter().map(|q| q.adjclose as f64).collect())
    } else {
        Ok(vec![])
    }
}

fn main() {
    let test = [1.5, 2.0, 3.0, 30.123, 123.0, 123.1];

    println!("{:?}", price_diff(&test));
}
