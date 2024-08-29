// https://github.com/wiseaidev/rust-data-analysis/blob/main/3-polars-tutorial-part-1.ipynb
#[allow(dead_code)]

use polars::datatypes::DataType;
use polars::prelude::*;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::sync::Arc;

fn main() {
   // first series
    let series1: Series = [1, 2, 3].iter().collect(); 
    println!("Series 1: {:?}", series1);

    // series with a name, observe the data is refering 
    // an array
    let seasons: Series = Series::new("seasons", 
        &["Winter", "Spring", "Summer", "Autumn"]);
    // println!("Series of season: {:?}", seasons);

    let null_series: Series = Series::new("null_seas", 
        &[None, Some(5), Some(1)]);
    // println!("Series of nulls: {:?}", null_series);

    // println!("null count is: {}", null_series.null_count());

    let null_dropped = null_series.drop_nulls();

    // println!("Null dropped series: {:?}", null_dropped);

    let simple_list = ["winter", "summer", "fall", "spring"];

    let from_array: Series = Series::new("from_array",&simple_list); 

    // println!("This is created from array: {:?}", from_array);

    let fltchnked = Float64Chunked::new("b_chnked", &[1., 2., 3.]).into_series();

    // println!("Float chunked: {:?}", fltchnked);

    let date: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 8, 29, 0, 0, 0).unwrap();
    let dt_series = Series::new("date_series", &[date.date_naive()]);

    // println!("Date Series: {:?}", dt_series);


    let house_items = ["bottle", "table", "bucket", "teletubby",
                        "speakerbox", "remote", "chair", "lift"];
    let item_series = Series::new("items", &house_items);

    // println!("{:?}", item_series);
    
    let slice1 = item_series.slice(0, 3); 

    // println!("Part of series: {:?}", slice1);

    // length, reverse, is_empty
    
    // println!("{:?}", slice1.len());
    // println!("{:?}", slice1.is_empty());
    // println!("{:?}", slice1.reverse());

    let msrs = [-1.02, 0.86, -4.68, 3.97, -0.63, -7.62, 5.42, -6.33];

    let msrs_series = Series::new("measurement", &msrs);

    // head, tail

    // println!("Heads are: {:?}", msrs_series.head(Some(5)));
    // head function is requiring a Option of usize
    
    // println!("Heads can give everyting is None: {:?}", msrs_series.head(None));

    // append, casting
    let ages_small = [5, 6, 7, 9, 8];
    let ages_sml_flt = ages_small.map(|x| x as f64 * 1.);
    let ages_big = [56.4, 78.8, 62.8, 65.7, 35.0];

    let mut age_sr1 = Series::new("age1", &ages_sml_flt);
    let age_sr2 = Series::new("age2", &ages_big);

    // println!("length of ages_sr1 before append: {:?}", age_sr1.len());
    // no neeed to assign the append process
    age_sr1.append(&age_sr2).unwrap();
    // println!("The appended series: {:?}", age_sr1);
    // println!("length of ages_sr1 after append: {:?}", age_sr1.len());
    
    // println!("Lets cast and print: {:?}", age_sr1.cast(&DataType::Int32).unwrap());

    // filling nulls, Fwd, Back, mean, min and max Fill

    let base_nulled = Series::new("some_nulled", &[Some(2), None, Some(12), Some(6), Some(34), None, None, Some(5)]);

    println!("Printing before starting fills: {:?}", base_nulled);

    let fwdfill = base_nulled.fill_null(FillNullStrategy::Forward(None)).unwrap();
    let bkfill = base_nulled.fill_null(FillNullStrategy::Backward(None)).unwrap();
    let minfill = base_nulled.fill_null(FillNullStrategy::Min).unwrap();
    let maxfill = base_nulled.fill_null(FillNullStrategy::Max).unwrap();
    let avgfill = base_nulled.fill_null(FillNullStrategy::Mean).unwrap();

    // println!("fill fwd: {:?}", fwdfill);
    // println!("fill bwd: {:?}", bkfill);
    // println!("fill minfill: {:?}", minfill);
    // println!("fill maxfill: {:?}", maxfill);
    // println!("fill avgfill: {:?}", avgfill);
    
    // Sampling and spreads
    //
    // println!("Sample 4 vals: {:?}", base_nulled.sample_n(4, true, true, Some(777)));

    // println!("Mean: {:?}", base_nulled.mean::<T>().unwrap());
    // println!("Max: {:?}", base_nulled.max::<T>().unwrap());
    // println!("Min: {:?}", base_nulled.min::<T>().unwrap());

    let qseries = base_nulled.quantile_as_series(0.75, QuantileInterpolOptions::Nearest).unwrap();

    println!("quantiles: {:?}", qseries);

}











