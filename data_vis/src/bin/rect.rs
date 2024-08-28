use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut backend = BitMapBackend::new("plotters-doc-data/1.png", (300, 200));
    backend.draw_rect((50, 50), (200, 150), &RED, true)?;
    backend.present()?;
    Ok(())
}
