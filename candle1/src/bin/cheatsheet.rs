use std::collections::HashMap;

use candle_core::{Tensor, DType, Device, IndexOp};
use anyhow::Result;

fn main() -> Result<()> {
    let dev = &Device::Cpu;

    let t1 = Tensor::new(&[[1f32, 2.], [3., 4.]], dev)?;

    println!("Lets see how t1 looks: {:?}", t1);

    let t2 = Tensor::zeros((2, 2), DType::F32, dev)?;

    println!("Lets see how t2 of zeros looks: {:?}", t2);

    // let t3 = t2.i((.., ..4))?; 

    // println!("Lets see how t3 indexed tensor looks: {:?}", t3);

    candle_core::safetensors::save(&HashMap::from([("A", t1)]), "models.safetensors")?;

    Ok(())
}
