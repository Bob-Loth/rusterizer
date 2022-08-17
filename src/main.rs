use crate::obj::{get_mesh_data, resize_obj};
use crate::space::{Fragment, Space};
use array2d::Array2D;
use std::num::NonZeroU32;
use tobj::Model;

mod io;
mod obj;
mod space;

fn parse_cmd() -> io::Args {
    match io::Args::new(std::env::args()) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("An error occurred during command line parsing: {:#?}", e);
            eprintln!("{}", io::Args::help());
            std::process::exit(1);
        }
    }
}

//the panic indicates a bug in error-handling for parse_cmd, or in Space's constructor.
fn create_space_transforms(width: NonZeroU32, height: NonZeroU32) -> space::Space {
    match space::Space::new(width, height) {
        Ok(space) => space,
        Err(e) => panic!("An error occurred during view volume creation: {:#?}", e),
    }
}

fn assemble_triangle() {}

fn rasterize(fragments: &mut Array2D<f32>, space: &Space, vertices: &[f32], indices: &[u32]) {
    assert_eq!(indices.len() % 3, 0);
    //for every triangle with coords x,y,z into window space
    for fragment in indices
        .chunks_exact(3)
        .map(|f| {
            (
                vertices[f[0] as usize],
                vertices[f[1] as usize],
                vertices[f[2] as usize],
            )
        })
        .map(|(x, y, z)| space.window_to_pixel(x, y, z))
    {

    }
}

fn main() {
    let args = parse_cmd();
    let space = create_space_transforms(args.image_width, args.image_height);
    //store triangle's indices and vertex positions into packed data structures.
    let mut models = get_mesh_data(&args.mesh_file);
    //positions proportionally scaled in the range [-1,1]
    resize_obj(&mut models);

    //maintain a z buffer, a 2d structure to store depth information per pixel.
    let mut fragments = Array2D::filled_with(
        f32::MAX,
        args.image_width.get() as usize,
        args.image_height.get() as usize,
    );
    //the actual rasterization operation.
    for model in models.iter() {
        rasterize(&mut fragments, &space,&model.mesh.positions, &model.mesh.indices);
    }

    //Points(x,y,z)
    //for each triangle(p1, p2, p3):
    //compute bounding box, as pair<Point, Point> or similar.
    //pre-compute static barycentric coordinates factor.
    //for each pixel in the bounding box:
    //compute barycentric coordinates alpha, beta, gamma.
    //if all (alpha, beta, gamma) >= 0 and <= 1:
    //color according to mode.

    //write out the data to an output file.
}
