use std::num::NonZeroU32;
use SpaceError::Init;

#[derive(Debug)]
pub struct Space {
    view_volume: ViewVolume,
    x_transform: PixelTransform,
    y_transform: PixelTransform,
}
#[derive(Debug)]
pub enum SpaceError {
    Init,
}

impl Space {
    pub fn new(width: NonZeroU32, height: NonZeroU32) -> Result<Space, SpaceError> {
        let vv = ViewVolume::new(width, height);
        Ok(Space {
            view_volume: vv.clone(),
            x_transform: PixelTransform::new(width, vv.left, vv.right).map_err(|_| Init)?,
            y_transform: PixelTransform::new(height, vv.bottom, vv.top).map_err(|_| Init)?,
        })
    }
    pub fn window_to_pixel(&self, x_window: f32, y_window: f32, z_window: f32) -> Fragment {
        Fragment {
            x: self.x_transform.window_to_pixel(x_window),
            y: self.y_transform.window_to_pixel(y_window),
            z: -z_window,
        }
    }
}

//a pixel with depth
pub struct Fragment {
    x: i32,
    y: i32,
    z: f32,
}

#[derive(Clone, Debug)]
struct ViewVolume {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl ViewVolume {
    //width and height
    fn new(width: NonZeroU32, height: NonZeroU32) -> ViewVolume {
        let h_w = height.get() as f32 / width.get() as f32;
        match (width, height) {
            _ if width < height => ViewVolume {
                left: -1.0,
                right: 1.0,
                top: h_w,
                bottom: -h_w,
            },
            _ => ViewVolume {
                left: -1.0 / h_w,
                right: 1.0 / h_w,
                top: 1.0,
                bottom: -1.0,
            },
        }
    }
}

//generates and contains shift and scale factors, given a pixel extent in a given dimension (height or width), and the viewing volume in that dimension.
#[derive(Debug, PartialEq)]
struct PixelTransform {
    shift: f32,
    scale: f32,
}
#[derive(Debug, PartialEq)]
pub(crate) enum PixelTransformError {
    BadViewVolume,
}

impl PixelTransform {
    fn new(
        pixel_extent: NonZeroU32,
        vv_min: f32,
        vv_max: f32,
    ) -> Result<PixelTransform, PixelTransformError> {
        if -vv_min != vv_max {
            Err(PixelTransformError::BadViewVolume)
        } else {
            let vv_diff = vv_max - vv_min;

            Ok(PixelTransform {
                shift: -vv_min * (pixel_extent.get() as f32 - 1.0) / vv_diff,
                scale: (pixel_extent.get() as f32 - 1.0) / vv_diff,
            })
        }
    }

    //flooring operation.
    fn window_to_pixel(&self, window_coord: f32) -> i32 {
        (self.scale * window_coord) as i32 + self.shift as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::space::PixelTransform;
    use std::num::NonZeroU32;

    mod pixel_transform {
        use super::{NonZeroU32, PixelTransform};
        use crate::space::PixelTransformError::BadViewVolume;
        #[test]
        fn pixel_bigger() {
            let p = PixelTransform::new(NonZeroU32::new(100).unwrap(), -0.5, 0.5).unwrap();
            assert_eq!(p.shift, 99.0 / 2.0); //how many pixels to move over. Starts at 0, ends at pixel_extent - 1.
            assert_eq!(p.scale, 99.0 / 1.0); //the number of pixels divided by the full extent of the viewing volume
        }
        #[test]
        fn pixel_smaller() {
            let p = PixelTransform::new(NonZeroU32::new(40).unwrap(), -50.0, 50.0).unwrap();
            assert_eq!(p.shift, 39.0 / 2.0); //how many pixels to move over. Starts at 0, ends at pixel_extent - 1.
            assert_eq!(p.scale, 39.0 / 100.0); //the number of pixels divided by the full extent of the viewing volume
        }
        // Occurrence implies a bad implementation of viewing volume, or a programming error in passing its data to PixelTransform::new().
        #[test]
        fn bad_input() {
            let p = PixelTransform::new(NonZeroU32::new(100).unwrap(), -10.0, 11.0);
            assert_eq!(p, Err(BadViewVolume))
        }
    }
}
