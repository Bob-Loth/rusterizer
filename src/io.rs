use std::num::NonZeroU32;

use crate::io::ArgsError::{ImageDimensions, ImageFile, MeshFile};

#[derive(Debug)]
pub struct Args {
    mesh_file: String,
    image_file: String,
    image_width: NonZeroU32,
    image_height: NonZeroU32,
    mode: Mode,
}

impl Args {
    pub fn help<'a>() -> &'a str {
        "Usage: rusterizer Meshfile Imagefile image_width image_height [-w | --wireframe]"
    }
}

impl Args {
    pub fn new(args: std::env::Args) -> Result<Args, ArgsError> {
        let mut iter = args.skip(1); //skip over the name of the executable.
        let input_mesh = iter.next().ok_or(MeshFile)?; //a missing argument
        let input_image = iter.next().ok_or(ImageFile)?;
        let width = iter
            .next()
            .ok_or(ImageDimensions("width missing"))?
            .parse::<NonZeroU32>()
            .map_err(|_| ImageDimensions("width invalid"))?;
        let height = iter
            .next()
            .ok_or(ImageDimensions("height missing"))?
            .parse::<NonZeroU32>()
            .map_err(|_| ImageDimensions("height invalid"))?;
        let input_mode = match iter.next().unwrap_or_default().as_str() {
            "--wireframe" | "-w" => Ok(Mode::Wireframe),
            "" => Ok(Mode::Depth),
            _ => Err(ArgsError::Mode), //something was there, but not a valid argument.
        }?;

        Ok(Args {
            mesh_file: input_mesh,
            image_file: input_image,
            image_width: width,
            image_height: height,
            mode: input_mode,
        })
    }
}

#[derive(Debug)]
pub enum ArgsError {
    MeshFile,
    ImageFile,
    ImageDimensions(&'static str),
    Mode,
}

#[derive(Debug)]
enum Mode {
    Depth,
    Wireframe,
}

#[cfg(test)]
mod struct_initialization {
    use super::*;

    #[test]
    fn valid_args<'a>() {
        let args = Args {
            mesh_file: String::from("a.obj"),
            image_file: String::from("a.png"),
            image_width: NonZeroU32::new(1).unwrap(),
            image_height: NonZeroU32::new(1).unwrap(),
            mode: Mode::Wireframe,
        };
    }
    #[test]
    #[should_panic]
    fn invalid_width() {
        let args = Args {
            mesh_file: String::from("a.obj"),
            image_file: String::from("a.png"),
            image_width: NonZeroU32::new(0).unwrap(),
            image_height: NonZeroU32::new(1).unwrap(),
            mode: Mode::Wireframe,
        };
    }
    #[test]
    #[should_panic]
    fn invalid_height() {
        let args = Args {
            mesh_file: String::from("a.obj"),
            image_file: String::from("a.png"),
            image_width: NonZeroU32::new(1).unwrap(),
            image_height: NonZeroU32::new(0).unwrap(),
            mode: Mode::Wireframe,
        };
    }

    #[test]
    fn no_args() {
        let args = std::env::args();
        if let Ok(_) = Args::new(args) {
            panic!("calling with no arguments should never return an Ok status.")
        }
    }
}
