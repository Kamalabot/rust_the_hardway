use candle_core::{DType, Device, Tensor};
use anyhow::Result;

fn main() {
    let data: [u32; 3] = [1u32, 2, 3];
    // let device = Device::new_cuda(0)?;
    let device = Device::Cpu;
    let tensor = Tensor::new(&data, &device).unwrap();
    println!("The tensor: {}", tensor);
}
