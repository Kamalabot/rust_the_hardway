#[allow(dead_code)]
#[allow(unused_imports)]

use either::Either;
use polars::prelude::*;
use std::path::Path;

fn read_data_frame_from_csv(
    csv_file_path: &Path,
) -> DataFrame {
    CsvReader::from_path(csv_file_path)
        .expect("Cannot open file.")
        .has_header(true)
        .finish()
        .unwrap()
}


fn main() {
    // https://www.kaggle.com/datasets/bhagatakash/airline2?select=Flight_on_time_HIX.csv

    let flight_path = Path::new("dataset/Flight_on_time_HIX.csv");
    let columns = ["Airline", "Origin_Airport", 
        "Destination_Airport", "Departure_Delay_Minutes",
        "Arrival_Delay_Minutes"];

    let flights_df: DataFrame = read_data_frame_from_csv(flight_path).select(columns).unwrap();
    println!("{:?}", flights_df.head(Some(2)));

    // need to use the latest lazy aggregation methods
    let arr_delay_mean_df: DataFrame = flights_df
        .group_by(["Airline"])
        .expect("Airline Column must exist!")
        .select(["Arrival_Delay_Minutes"])
        .mean()
        .unwrap();
    println!("Some group data: {:?}", arr_delay_mean_df.head(Some(5)));

    let dfa1: DataFrame = df!("Carrier" => &["HA", "EV", "VX", "DL"],
                         "ArrDelay" => &[-3, 28, 0, 1]).unwrap();
    let dfa2: DataFrame = df!("Airline" => &["HA", "EV", "OO", "VX"],
                             "DepDelay" => &[21, -8, 11, -4]).unwrap();
    
    let df3: DataFrame = dfa1.join(&dfa2, ["Carrier"], ["Airline"],
        JoinType::Inner.into()).unwrap();
    // or: let df3: DataFrame = df1.inner_join(&df2, ["Carrier"], ["Airline"]).unwrap();

    println!("Some group data: {:?}",df3.head(Some(5)));

    let df3: DataFrame = dfa1.left_join(&dfa2, ["Carrier"], ["Airline"]).unwrap();
    println!("Some group data: {:?}", df3.head(Some(5)));
    
    let df5: DataFrame = dfa1.outer_join(&dfa2, ["Carrier"], ["Airline"]).unwrap();
    println!("Some group data: {:?}", df5.head(Some(5)));
}






