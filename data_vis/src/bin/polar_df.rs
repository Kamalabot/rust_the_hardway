#[allow(unused_imports)]
#[allow(dead_code)]

use std::path::Path;
use polars::prelude::*;
use either::Either;

fn main() {
    let df = DataFrame::default();
    // println!("Default DF: {:?}", df);
    
    let s1 = Series::new("name", &["tv", "smartphone", "remote", "shoes", "fridge"]);
    let s2 = Series::new("age", &[32, 6, 67, 23, 46]);
    let s3 = Series::new("weight", &[3632, 216, 86, 32, 433]);
    let cost = Series::new("cost", &[325, 22, 6, 932, 43]);

    let items_df: DataFrame = DataFrame::new(vec![s1, s2, s3]).unwrap();

    // println!("more involved items_df: {:?}", items_df);
    
    let categ_df: DataFrame = df!("categ" => &["d", "e", "f"],
                                    "numeric" => &[1, 2, 3],
                                    "object" => &["a", "b", "c"]).unwrap();

    // println!("Categorical= {:?}", categ_df);
    //
    // println!("Head: {:?}", categ_df.head(Some(1)));
    // can take Some(n) and Non

    // The s1, s2, s3 are moved into the df, so can re-create
    let s1 = Series::new("name", &["tv", "smartphone", "remote", "shoes", "fridge"]);
    let s2 = Series::new("age", &[32, 6, 67, 23, 46]);
    let s3 = Series::new("weight", &[3632, 216, 86, 32, 433]);
    let cost = Series::new("cost", &[325, 22, 6, 932, 43]);

    let updtdf = df!("Name" => &s1, "age" => &s2,
                    "weight"=> &s3, "cost" => &cost).unwrap();


    // println!("Name series: {:?}", &updtdf["Name"]);

    // println!("Two cols: {:?}", updtdf.select(["name", "age"]).unwrap());

    let updtdf1 = df!("Name" => &s1, "age" => &s2,
                    "weight"=> &s3, "cost" => &cost).unwrap();
    
    let filter_age = updtdf1.column("age").expect("Age col is missing").gt(35).unwrap();
    let fil_age_df = updtdf1.filter(&filter_age).unwrap();

    // println!("Filtered ages: {:?}", fil_age_df);
    // 
    // println!("Can we slice the df: {:?}", updtdf1.slice(2, 3));
    // yes, the dataframes are not moved. They are in the scope

    let tposed = updtdf1.transpose(Some("old_cols"),
        Some(Either::Right(vec!["Item1".to_string(),
            "Item2".to_string(),
            "Item3".to_string(),
            "Item4".to_string(),
            "Item5".to_string()
        ])
        )).unwrap();

    println!("Can we transpose the df: {:?}", tposed);
    // counting nulls

    let df = df!("Name" => &[Some("Mahmoud"),  None, None],
             "Age" => &[22, 25, 36],
             "Gender" => &["M", "M", "M"],
             "Salary" => &[50000, 60000, 250000]).unwrap();

    println!("Count of null: {:?}",df.null_count());

    let df = df!("Name" => &["Mahmoud",  "Mahmoud", "ThePrimeagen"],
             "Age" => &[22, 22, 36],
             "Gender" => &["M", "M", "M"],
             "Salary" => &[50000, 50000, 250000]).unwrap();

    // duplicate identifier masks
    let dupmask = df.is_duplicated().unwrap();
    // unique value mask
    let uniqmask = df.is_unique().unwrap();
    
    let dup_removed = df.filter(&dupmask).unwrap();

    println!("Duplicate removed: {:?}",dup_removed);

    let df_color: DataFrame = df!("Fruit" => &["Apple", "Apple", "Pear"],
                                "Color" => &["Red", "Yellow", "Green"]).unwrap();

    let droped_colr = df_color.drop("Color").unwrap(); 
    // we can do drop_in_place("Color") too
    // and drop_many(&["Color", ""])

    println!("Color dropped: {:?}",droped_colr);
    let df_color_1: DataFrame = df!("Fruit" => &["Apple", "Apple", "Pear"],
                                "Color" => &[Some("Red"), None, None]).unwrap();
    
    let df_cln = df_color_1.drop_nulls::<String>(None).unwrap();
    
    println!("{:?}", df_cln);

    let df = df!("Name" => &["Mahmoud", "Ali", "ThePrimeagen"],
             "Age" => &[22, 25, 36],
             "Gender" => &["M", "M", "M"],
             "Salary" => &[Some(50000), Some(60000), None]).unwrap();
    let mask_sal = df.column("Salary").expect("Salary missing").is_not_null(); 
    // extract only rows wheree 
    println!("Mask of Salary: {:?}", mask_sal);

    println!("Means: {:?}", df.mean());
    println!("Median: {:?}", df.median());
    println!("Std: {:?}", df.std(1));
}   
















