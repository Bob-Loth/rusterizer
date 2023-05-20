use crate::io::Args;
use crate::obj::{get_mesh_data, resize_obj};
use crate::point::rasterize;
use crate::space::Fragment;
use array2d::Array2D;
use png::Writer;

use std::fs::File;
use std::io::BufWriter;
use std::num::NonZeroU32;
use std::path::Path;
use std::process;

//use crate::point::rasterize;

mod io;
mod obj;
mod point;
mod space;

fn parse_cmd() -> Args {
    match Args::new(std::env::args()) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("An error occurred during command line parsing: {:#?}", e);
            eprintln!("{}", Args::help());
            process::exit(1);
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
        rasterize(
            &mut fragments,
            &space,
            &model.mesh.positions,
            &model.mesh.indices,
            args.mode,
        );
    }

    let mut writer = get_writer(&args);

    let pixels: Vec<f32> = fragments
        .elements_row_major_iter()
        .map(|&f| if f > 1.0 { 1.0 } else { f })
        .collect();
    for p in &pixels {
        assert!((-1.0f32..=1.0f32).contains(p));
    }

    let mut data = [165u8, 255u8, 214u8, 255u8]
        .repeat((args.image_width.get() * args.image_height.get()) as usize); // An array containing a RGBA sequence.

    for i in 0..pixels.len() {
        //pixels = [-1, 1]
        // -    -> [1 ,-1]
        // +1   -> [2 , 0]
        // /2   -> [1 , 0]
        // *data-> [255,0]

        data[i * 4] = (((-pixels[i] + 1.0) / 2.0) * (data[i * 4] as f32)) as u8;
        data[i * 4 + 1] = (((-pixels[i] + 1.0) / 2.0) * (data[i * 4 + 1] as f32)) as u8;
        data[i * 4 + 2] = (((-pixels[i] + 1.0) / 2.0) * (data[i * 4 + 2] as f32)) as u8;
        data[i * 4 + 3] = (((-pixels[i] + 1.0) / 2.0) * (data[i * 4 + 3] as f32)) as u8;
    }
    
    for item in &pixels {
        println!("{}", item);
    }

    println!("wrote to: {}", args.image_file);
    writer.write_image_data(&data).unwrap(); // Save
}

fn get_writer(args: &Args) -> Writer<BufWriter<File>> {
    let path = Path::new(&args.image_file);
    let create = File::create(path);
    if create.is_err() {
        eprintln!(
            "an error happened when attempting to {}: {}",
            args.image_file,
            create.unwrap_err()
        );
        process::exit(1);
    }
    let w = BufWriter::new(create.unwrap());
    let mut encoder = png::Encoder::new(w, args.image_width.get(), args.image_height.get());
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);

    encoder.write_header().unwrap()
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
