// https://github.com/wiseaidev/rust-data-analysis/blob/main/3-polars-tutorial-part-1.ipynb
use polars::datatypes::DataType;
use polars::prelude::*;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::sync::Arc;

fn main() {
   // first series
    let series1: Series = [1, 2, 3].iter().collect(); 
    println!("Series 1: {:?}", series1);

    // series with a name
    let seasons: Series = Series::new("seasons", &["Winter", "Spring", "Summer", "Autumn"]);
    println!("Series of season: {:?}", seasons);

    let null_series: Series = Series::new("null_seas", &[None, Some(5), Some(1)]);
    println!("Series of nulls: {:?}", null_series);

    println!("null count is: {}", null_series.null_count());

    let null_dropped = null_series.drop_nulls();

    println!("Null dropped series: {:?}", null_dropped);


}
