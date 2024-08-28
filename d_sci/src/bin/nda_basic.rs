// Starting with ArrayBase doc
use ndarray::{
    Array3,
    Array2,
    Array1,
    Array,
    s,
    Axis,
};

fn main() {
    let mut temp = Array3::<f64>::zeros((2,2,2));
    println!("Lets see the array: {:?}", temp);

    temp[[1, 1, 1]] += 0.5;

    // println!("Lets see the array: {:?}", temp);

    // println!("The shape of array: {:?}", temp.shape());

    // println!("The strides of array: {:?}", temp.strides());
    // strides determine how elements are laid out in 
    // memory. How far in terms of memory addresses you 
    // need to move to reach next element

    // start by creating a 1-d array, and check its stride

    let arr_1d = Array1::from_vec(vec![1, 2, 3, 4, 5]);

    println!("1d array is: {:?}", arr_1d);
    // stride will be 1 memory address
    
    let mut arr_2d = Array2::from_shape_vec((2, 3), vec![1, 2, 3, 4, 5, 6]).unwrap();

    println!("2D array, without providing shape: {:?}", arr_2d);
    
    println!("2D array, stride: {:?}", arr_2d.strides());

    // stride = [3, 1] as to reach the next row (element)
    // three array elements have to be skipped.
    
    let _arr_v1 = arr_2d.view();
    // the source array has to be mutable
    let mut _arr_v2 = arr_2d.view_mut();
    // understand how slicing works
    // let arr_sl1 = arr_2d.slice((0, 1));
    // Dimensions:
    // greatest dimension list first, most rapidly varying index at last
    // Array2 < here 2 is called D type parameter 
    //
    // println!("Accessing 1st row * 2rd col: {:?}", arr_2d[[0, 2]]);

    // println!("What is raw_dim of arr_2d: {:?}", arr_2d.raw_dim());
    // 
    // let array1: Array1<f64> = Array::zeros(5); // 1d 5 elem array
    // //
    // println!("1d array of {:?}", array1);
    // // println!("1d array of {:?}", array1);
    // 
    // let array2: Array2<f64> = Array::ones((2, 3)); // 2d array with 1s
    // 
    // println!("2d array of {:?}", array2);

    // let rng_arr: Array1<f64> = Array::linspace(0., 1., 5);
    // // 5 values between 0 and 1

    // println!("Linear Array spaced: {:?}", rng_arr);

    // let array5: Array2<f64> = Array::eye(3);

    // println!("Identity Array {:?}", array5);
    // 
    println!("*****Slicing operations:*****");
    println!("mapv, slice, s!, permuted_axes, clone");
    println!("into_shape_with_order");

    // let rng_arr1: Array1<f64> = Array::linspace(1.0, 9.0, 9);

    // let rshap2d = rng_arr1.clone().into_shape_with_order((3, 3)).unwrap();
    // 
    // println!("rhap2d values are: {:?}", rshap2d);

    // let slc = rshap2d.slice(s![0..2, 1..3]);

    // println!("Slice value is: {:?}", slc);

    // let slc1 = rng_arr1.slice(s![1..3]);

    // println!("1D slice of the rng_arr1 is: {:?}", slc1);

    // println!("Bringing the 2d back to 1D");

    // let rshap1d = rshap2d.clone().into_shape_with_order((1, 9)).unwrap();

    // println!("Unwrapped 1D array: {:?}", rshap1d);

    // println!("rshap2d before transpose: {:?}", rshap2d);

    // let tposed = rshap2d.permuted_axes((1, 0));
    // // this actually takes lot of shifting around
    // println!("Permuted after that: {:?}", tposed);

    // let dbl_tposed = tposed.mapv(|x| x * 2.0);

    // println!("doubled data: {:?}", dbl_tposed);

    println!("**********Starting Math Operation**********");

    let arr1: Array2<i32> = Array::from_shape_vec((2, 2), vec![1, 2, 3, 4,]).unwrap();
    let arr2: Array2<i32> = Array::from_shape_vec((2, 2), vec![15, 62, 83, 94,]).unwrap();

    let dotpdt = arr1.dot(&arr2);

    println!("Dot Product: {:?}", dotpdt);

    println!("Concatenating arrays");
    let concated: Array2<i32> = ndarray::concatenate(Axis(0), &[arr1.view(), arr2.view()]).unwrap();

    println!("Concatenated array: {:?}", concated);
    
    let sum_axis = concated.sum_axis(Axis(0));

    println!("Sum Axis is: {:?}", sum_axis);
}








