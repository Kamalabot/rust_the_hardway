use ndarray::{Array, Array2, Array3, Axis, AxisDescription};
use ndarray::parallel::prelude::*;
use ndarray::Zip;

fn main() {
    let mut a = Array2::<f64>::zeros((2, 2));

    a.par_map_inplace(|x| *x = x.exp());
    // same as 
    a.par_mapv_inplace(f64::exp);
    // same as 
    a.par_iter_mut().for_each(|x| println!("{}", x));

    // getting row sum
    let b = Array::linspace(0., 63., 64).into_shape_with_order((4, 16)).unwrap();

    let mut sums = Vec::new();

    b.axis_iter(Axis(0))
        .into_par_iter()
        .map(|row| row.sum())
        .collect_into_vec(&mut sums);
    
    println!("Vectors of sums: {:?}", sums);

    let mut shapes = Vec::new();

    b.axis_chunks_iter(Axis(0), 3)
        .into_par_iter()
        .map(|chk| chk.shape().to_owned())
        .collect_into_vec(&mut shapes);

    println!("Extracts 3, 16 and 1, 16 {:?}", shapes);
    // lockstep function appln for multiple arrays
    
    type Array3f64 = Array3<f64>; 

    const N:usize = 128;
    // huge array with 1.0
    let a1 = Array3f64::from_elem((N, N, N), 1.);
    // same shape, but with 2.0
    let b1 = Array3f64::from_elem(a1.dim(), 2.); 
    // another with 0
    let mut c1 = Array3f64::zeros(a1.dim());


    Zip::from(&mut c1)
        .and(&a1)
        .and(&b1)
        .par_for_each(|c1, &a1, &b1| {
            *c1 += a1 - b1;
        });

    println!("Getting at c1: {:?}", c1);

}   
