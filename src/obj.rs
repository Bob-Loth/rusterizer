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

#[derive(PartialEq, Debug)]
struct MinMaxValues {
    min_x: f32,
    min_y: f32,
    min_z: f32,
    max_x: f32,
    max_y: f32,
    max_z: f32,
}

impl Default for MinMaxValues {
    fn default() -> Self {
        MinMaxValues {
            min_x: f32::MAX,
            min_y: f32::MAX,
            min_z: f32::MAX,
            max_x: f32::MIN,
            max_y: f32::MIN,
            max_z: f32::MIN,
        }
    }
}

fn get_min_max_chunks(model: &Model) -> MinMaxValues {
    let x = model
        .mesh
        .positions
        .chunks_exact(3)
        .fold(MinMaxValues::default(), |mut acc, p| {
            acc.max_x = p[0].max(acc.max_x);
            acc.max_y = p[1].max(acc.max_y);
            acc.max_z = p[2].max(acc.max_z);
            acc.min_x = p[0].min(acc.min_x);
            acc.min_y = p[1].min(acc.min_y);
            acc.min_z = p[2].min(acc.min_z);
            acc
        });
    x
}

fn resize_obj(obj: &[Model]) {
    //find min and max of each dimension x,y,z
    for model in obj.iter() {
        let (min_x, max_x) = get_min_max(model, 0);
        let (min_y, max_y) = get_min_max(model, 1);
        let (min_z, max_z) = get_min_max(model, 2);
        println!(
            "{} {} {} {} {} {}",
            min_x, max_x, min_y, max_y, min_z, max_z
        );
        let min_max_values = get_min_max_chunks(model);
        assert_eq!(
            MinMaxValues {
                min_x,
                min_y,
                min_z,
                max_x,
                max_y,
                max_z,
            },
            min_max_values
        )
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

        assert!(mesh.positions.len() % 3 == 0);
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
