use crate::space;
use space::Transform;
use std::iter::{Skip, StepBy};
use std::slice::{Iter, IterMut};
use tobj::load_obj;
use tobj::Model;

pub(crate) fn get_mesh_data(handle: &str) -> Vec<Model> {
    let (mut models, _mats_result) =
        load_obj(handle, &tobj::LoadOptions::default()).expect("obj load error");
    resize_obj(&mut models);
    models
}

fn get_min_max(model: &Model, offset: usize) -> (f32, f32) {
    get_vertices_of_dim(model, offset).fold((f32::MAX, -f32::MAX), |acc, &x| {
        (acc.0.min(x), acc.1.max(x))
    })
}

//returns an iterator over a given dimension offset (0,1,2) referring to (x,y,z)
fn get_vertices_of_dim(model: &Model, offset: usize) -> StepBy<Skip<Iter<f32>>> {
    model.mesh.positions.iter().skip(offset).step_by(3)
}

fn get_mut_vertices_of_dim(model: &mut Model, offset: usize) -> StepBy<Skip<IterMut<f32>>> {
    model.mesh.positions.iter_mut().skip(offset).step_by(3)
}

//modifies mesh positions in-place to be in the range [-1,1].
pub(crate) fn resize_obj(obj: &mut [Model]) {
    for model in obj.iter_mut() {
        //find min and max of each dimension x,y,z
        let x = get_min_max(model, 0);
        let y = get_min_max(model, 1);
        let z = get_min_max(model, 2);
        //from these, compute necessary shift and scale for each dimension
        let max_extent = get_max_extent(x, y, z);
        let x_transform = Transform::from_extent(x.0, max_extent);
        let y_transform = Transform::from_extent(y.0, max_extent);
        let z_transform = Transform::from_extent(z.0, max_extent);
        //shift and scale all vertices.

        get_mut_vertices_of_dim(model, 0).for_each(|f| *f = x_transform.apply(*f));
        get_mut_vertices_of_dim(model, 1).for_each(|f| *f = y_transform.apply(*f));
        get_mut_vertices_of_dim(model, 2).for_each(|f| *f = z_transform.apply(*f));
    }
}
//return the widest difference in minimum and maximum's across all 3 dimensions.
fn get_max_extent(x: (f32, f32), y: (f32, f32), z: (f32, f32)) -> f32 {
    (x.1 - x.0).max(y.1 - y.0).max(z.1 - z.0)
}
//return shift and scale factors

impl Transform {
    fn from_extent(min: f32, extent: f32) -> Transform {
        Transform {
            extent: extent as u64,
            scale: 2.0 / extent,
            shift: min + (extent / 2.0),
        }
    }
    fn apply(&self, dimension: f32) -> f32 {
        (dimension - self.shift) * self.scale
    }
}

#[cfg(test)]
mod tests {

    use crate::obj::get_min_max;
    use crate::resize_obj;
    use tobj::{load_obj, LoadOptions};

    #[test]
    fn chunked_access_eq_to_skipping_access() {
        let (obj, _mats) =
            load_obj("./tests/resources/teapot.obj", &LoadOptions::default()).unwrap();
        for model in &obj {
            let (min_x, max_x) = get_min_max(model, 0);
            let (min_y, max_y) = get_min_max(model, 1);
            let (min_z, max_z) = get_min_max(model, 2);
            println!(
                "{min_x} {max_x} {min_y} {max_y} {min_z} {max_z}"
            );
            assert!(min_x <= max_x);
            assert!(min_y <= max_y);
            assert!(min_z <= max_z);
        }
    }

    #[test]
    fn resize() {
        let (mut after, _mats) =
            load_obj("./tests/resources/teapot.obj", &LoadOptions::default()).unwrap();
        let before = after.clone();
        resize_obj(&mut after);
        for (b, a) in before.iter().zip(after.iter()) {
            assert_ne!(b.mesh.positions, a.mesh.positions);
            assert!(a.mesh.positions.iter().all(|&f| (-1.0..=1.0).contains(&f)));
        }
    }
}
