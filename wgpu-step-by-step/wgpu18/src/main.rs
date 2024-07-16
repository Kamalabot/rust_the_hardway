mod common;
mod math_func;
#[path="surface_data.rs"]
mod sd;
use std::f32::consts::PI;

fn main(){
    let mut function_selection = 0;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1{
        function_selection = args[1].parse().unwrap();
    }

    let ps_struct:sd::ParametricSurface;

    if function_selection == 1 {
        ps_struct = sd::ParametricSurface {f:math_func::klein_bottle, umin:0.0, umax:PI, vmin:0.0, vmax:2.0*PI,
            u_segments:120, v_segments:40, scale: 1.0,..Default::default()};
    } else if function_selection == 2 {
        ps_struct = sd::ParametricSurface{f:math_func::wellenkugel, umin:0.0, umax:14.5, vmin:0.0, vmax:5.0,
            u_segments:100, v_segments:50, scale:0.17, colormap_name:"cool", ..Default::default()};
    } else {
        ps_struct = sd::ParametricSurface{..Default::default()};
    }

    let (pos_data, normal_data, color_data, index_data) 
        = sd::ParametricSurface::new(ps_struct);

    let light_data = common::light([1.0, 1.0, 1.0], 0.1, 0.8, 0.4, 30.0, 1);

    common::run(&pos_data, &normal_data, &color_data, &index_data, light_data);
}