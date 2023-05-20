use std::num::NonZeroU32;

#[derive(Debug, PartialEq)]
pub(crate) struct Args {
    pub(crate) mesh_file: String,
    pub(crate) image_file: String,
    pub(crate) image_width: NonZeroU32,
    pub(crate) image_height: NonZeroU32,
    pub(crate) mode: Mode,
}

impl Args {
    pub(crate) fn help<'a>() -> &'a str {
        "Usage: rusterizer Meshfile Imagefile image_width image_height [-w | --wireframe]"
    }
}

impl Args {
    pub(crate) fn new(args: std::env::Args) -> Result<Args, ArgsError> {
        let unstructured_args: Vec<String> = args.collect();
        Self::structure_args(&unstructured_args)
    }

    fn structure_args<T: AsRef<str>>(args: &[T]) -> Result<Args, ArgsError> {
        if !(args.len() == 5 || args.len() == 6) {
            //neither nor
            return Err(ArgsError::BadLength);
        }

        let input_mesh = args[1].as_ref();
        let input_image = args[2].as_ref();
        let width = args[3]
            .as_ref()
            .parse::<NonZeroU32>()
            .map_err(|_| ArgsError::ImageDimensions("width invalid"))?;
        let height = args[4]
            .as_ref()
            .parse::<NonZeroU32>()
            .map_err(|_| ArgsError::ImageDimensions("height invalid"))?;
        let mode = if args.len() == 6 {
            match args[5].as_ref() {
                "--wireframe" | "-w" => Ok(Mode::Wireframe),
                _ => Err(ArgsError::BadMode), //something was there, but not a valid argument.
            }?
        } else {
            Mode::Depth
        };

        Ok(Args {
            mesh_file: String::from(input_mesh),
            image_file: String::from(input_image),
            image_width: width,
            image_height: height,
            mode,
        })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum ArgsError {
    BadLength,
    ImageDimensions(&'static str),
    BadMode,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    Depth,
    Wireframe,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_args() {
        let _args = Args {
            mesh_file: String::from("a.obj"),
            image_file: String::from("a.png"),
            image_width: NonZeroU32::new(1).unwrap(),
            image_height: NonZeroU32::new(1).unwrap(),
            mode: Mode::Wireframe,
        };
    }

    #[test]
    fn invalid_height() {
        let raw_args = vec!["name", "a", "b", "1", "-1"];
        let args = Args::structure_args(&raw_args);
        assert_eq!(args, Err(ArgsError::ImageDimensions("height invalid")));
    }

    #[test]
    fn invalid_width() {
        let raw_args = vec!["name", "a", "b", "0", "1"];
        let args = Args::structure_args(&raw_args);
        assert_eq!(args, Err(ArgsError::ImageDimensions("width invalid")));
    }

    #[test]
    fn invalid_mode() {
        let raw_args1 = vec!["name", "a", "b", "1", "1", ""];
        let args1 = Args::structure_args(&raw_args1);
        assert_eq!(args1, Err(ArgsError::BadMode));

        let raw_args2 = vec!["name", "a", "b", "1", "1", "garbage"];
        let args2 = Args::structure_args(&raw_args2);
        assert_eq!(args2, Err(ArgsError::BadMode));
    }

    #[test]
    fn wireframe_short() {
        let raw_args = vec!["name", "a", "b", "1", "1", "-w"];
        let args = Args::structure_args(&raw_args);
        assert_eq!(args.unwrap().mode, Mode::Wireframe);
    }
    #[test]
    fn wireframe_fully_qualified() {
        let raw_args = vec!["name", "a", "b", "1", "1", "--wireframe"];
        let args = Args::structure_args(&raw_args);
        assert_eq!(args.unwrap().mode, Mode::Wireframe)
    }

    #[test]
    fn depth() {
        let raw_args = vec!["name", "a", "b", "1", "1"];
        let args = Args::structure_args(&raw_args);
        assert_eq!(args.unwrap().mode, Mode::Depth);
    }

    #[test]
    fn no_args() {
        let args = std::env::args();
        if Args::new(args).is_ok() {
            panic!("calling with no arguments should never return an Ok status.")
        }
    }
}
