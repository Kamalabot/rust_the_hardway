use ndarray::{Array, Array1, Array2, s};
use ndarray::parallel::prelude::*;
// need to run cargo add ndarray-parallel

fn main() {
    let array: Array2<i32> = Array::from_shape_vec((2, 3), vec![1, 2, 3, 4, 5, 6]).unwrap();

    for elem in array.iter() {
        println!("elem: {}", elem)
        // it goes thru all elem, like a list
    }
    
    // like enumerate
    for (idx, elem) in array.indexed_iter(){
        println!("elem_{:?} is {}", idx, elem);
        // will be surprised at the idx value
    }
    // getting the entire row 
    for row in array.outer_iter() {
        println!("Row: {:?}", row);
    }

    let mut arr_mut: Array2<i32> = array.clone();
    // get a copy as slice
    let mut slice = arr_mut.slice_mut(s![0, ..]);
    slice.fill(10);
    
    println!("Modified array filled with 10: {:?}", slice);

    let src: Array1<i32> = Array::from_vec(vec![1, 2, 3]);
    let des: Array2<i32> = Array::from_shape_vec((3, 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        .unwrap();
    
    let bcst = src + des;

    println!("Resulting bcst: {:?}", bcst);
}
