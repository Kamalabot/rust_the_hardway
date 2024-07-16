mod common;
mod vertex_data;

fn create_vertices(r_torus: f32, r_tube: f32, n_torus: usize, n_tube:usize) -> Vec<common::Vertex> {
    let(pos, normal) = vertex_data::torus_data(r_torus, r_tube, n_torus, n_tube);
    let mut data:Vec<common::Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(common::vertex(pos[i], normal[i]));
    }
    data.to_vec()
}

fn main(){
    let vertex_data = create_vertices(1.5, 0.4, 100, 50);
    let light_data = common::light([1.0,0.0,0.0], [1.0, 1.0, 0.0], 0.1, 0.6, 0.3, 30.0);
    common::run(&vertex_data, light_data, "torus");
}
