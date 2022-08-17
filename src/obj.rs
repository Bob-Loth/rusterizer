use tobj::load_obj;
use tobj::Model;



pub(crate) fn get_mesh_data(handle: &str) -> Vec<Model> {
    let (models, _mats_result) =
        load_obj(handle, &tobj::LoadOptions::default()).expect("obj load error");

    print_obj(&models);
    resize_obj(&models);
    models
}

fn get_min_max(model: &Model, offset: usize) -> (f32, f32) {
    model
        .mesh
        .positions
        .iter()
        .skip(offset)
        .step_by(3)
        .fold((f32::MAX, -f32::MAX), |acc, &x| {
            (acc.0.min(x), acc.1.max(x))
        })
}


fn resize_obj(obj: &[Model]) {
    //find min and max of each dimension x,y,z
    for model in obj.iter() {
        let (min_x, max_x) = get_min_max(model, 0);
        let (min_y, max_y) = get_min_max(model, 1);
        let (min_z, max_z) = get_min_max(model, 2);

    }
    //from these, compute necessary shift and scale for each dimension

    //shift and scale all vertices.
}

fn print_obj(obj: &[Model]) {
    println!("number of models: {}", obj.len());
    for (i, m) in obj.iter().enumerate() {
        let mesh = &m.mesh;

        println!("model[{}].name = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        println!(
            "Size of model[{}].face_arities: {}",
            i,
            mesh.face_arities.len()
        );

        let mut next_face = 0;
        for f in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
            println!("    face[{}] = {:?}", f, face_indices);
            next_face = end;
        }

        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);

        assert_eq!(mesh.positions.len() % 3, 0);
        for v in 0..mesh.positions.len() / 3 {
            println!(
                "    v[{}] = ({}, {}, {})",
                v,
                mesh.positions[3 * v],
                mesh.positions[3 * v + 1],
                mesh.positions[3 * v + 2]
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use criterion::Criterion;
    use tobj::{load_obj, LoadOptions, Model};
    use crate::obj::get_min_max;
    #[test]
    fn chunked_access_eq_to_skipping_access(){
        let (obj, _mats) = load_obj("./tests/resources/teapot.obj", &LoadOptions::default()).unwrap();
        for model in obj.iter() {
            let (min_x, max_x) = get_min_max(model, 0);
            let (min_y, max_y) = get_min_max(model, 1);
            let (min_z, max_z) = get_min_max(model, 2);
            println!(
                "{} {} {} {} {} {}",
                min_x, max_x, min_y, max_y, min_z, max_z
            );
        }
    }

    #[test]
    fn bench_skip(){
        let (obj,_mats) = load_obj("./tests/resources/teapot.obj",&LoadOptions::default()).unwrap();

        for model in obj.iter() {
            let (min_x, max_x) = get_min_max(model, 0);
            let (min_y, max_y) = get_min_max(model, 1);
            let (min_z, max_z) = get_min_max(model, 2);
        }
    }


}
