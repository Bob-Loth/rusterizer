extern crate core;

mod io;

fn main() {
    //command arguments: meshfile imagefile imagewidth imageheight mode
    let args = match io::Args::new(std::env::args()) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("An error occurred during command line parsing: {:#?}", e);
            eprintln!("{}", io::Args::help());
            std::process::exit(1);
        }
    };

    //extract information from the mesh file, resize if necessary
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
