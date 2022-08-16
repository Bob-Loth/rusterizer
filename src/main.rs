extern crate core;

use crate::obj::get_mesh_data;
use std::num::NonZeroU32;

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

fn main() {
    let args = parse_cmd();
    let space = create_space_transforms(args.image_width, args.image_height);
    let models = get_mesh_data(&args.mesh_file);

    //store triangle's indices and vertex positions into packed data structures.

    //maintain a z buffer, a 2d structure to store depth information per pixel.

    //the actual rasterization operation.
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
