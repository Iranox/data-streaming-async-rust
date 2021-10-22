use chrono::prelude::*;
use std::io::{Error, ErrorKind};
use clap::{Arg, App};
use yahoo_finance_api as yahoo;

fn max(series: &[f64]) -> Option<f64> {
    if !series.is_empty() {
        return Some(series.iter().fold(f64::MIN, |pre, suc| pre.max(*suc)));
    }
    None
}

fn min(series: &[f64]) -> Option<f64> {
    if !series.is_empty() {
        return Some(series.iter().fold(f64::MAX, |pre, suc| pre.min(*suc)));
    }
    None
}

fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    if series.is_empty() {
        return None;
    }

    let (last_price, first_price) = (series.last().unwrap(), series.first().unwrap());
    let price_diff_absolute = last_price - first_price;
    let first = if *first_price == 0.0 {
        1.0
    } else {
        *first_price
    };
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
    let matches = App::new("My Test Program")
    .version("0.1.0")
    .author("Hackerman Jones <hckrmnjones@hack.gov>")
    .about("Teaches argument parsing")
    .arg(Arg::with_name("from")
             .short("f")
             .long("from")
             .takes_value(true)
             .help("start Date"))
    .arg(Arg::with_name("Symbol")
             .short("s")
             .long("symbols")
             .takes_value(true)
             .help("Five less than your favorite number"))
    .get_matches();

   let current_date = Utc::now();
   let  from = matches.value_of("from");
   let  symbol = matches.value_of("Symbol");

   if from.is_some() && symbol.is_some()  {
    let datetime = Utc.from_utc_datetime(&DateTime::parse_from_rfc3339(from.unwrap()).unwrap().naive_utc());
    let t =  String::from(symbol.unwrap());
    let split =t.split(",");
    println!("Symbol; max; min; price");
    for s in split {
        let closes = fetch_closing_data(&s, &datetime, &current_date);
        if closes.is_ok() {
            let value = closes.unwrap();
           println!("{};{};{};{}", s, max(&value).unwrap(), min(&value).unwrap(), *value.last().unwrap());
        } 
    }

}

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_max() {
        let test_values = [1.5, 2.0, 3.0, 30.123, 123.0, 123.1];
        assert_eq!(max(&test_values), Some(123.1));
    }

    #[test]
    fn test_max_with_empty_input() {
        assert_eq!(max(&vec![]), None);
    }

    #[test]
    fn test_min() {
        let test_values = [1.5, 2.0, 3.0, 30.123, 123.0, 123.1];
        assert_eq!(min(&test_values), Some(1.5));
    }

    #[test]
    fn test_min_without_input() {
        assert_eq!(min(&vec![]), None);
    }
}
