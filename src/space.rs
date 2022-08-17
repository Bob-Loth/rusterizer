use std::num::NonZeroU32;
use SpaceError::Init;

#[derive(Debug)]
pub struct Space {
    view_volume: ViewVolume,
    x_transform: Transform,
    y_transform: Transform,
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
            x_transform: Transform::new(width, vv.left, vv.right).map_err(|_| Init)?,
            y_transform: Transform::new(height, vv.bottom, vv.top).map_err(|_| Init)?,
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
#[derive(Clone)]
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
pub struct Transform {
    pub(crate) shift: f32,
    pub(crate) scale: f32,
}
#[derive(Debug, PartialEq)]
pub(crate) enum PixelTransformError {
    BadViewVolume,
}

impl Transform {
    fn new(
        pixel_extent: NonZeroU32,
        vv_min: f32,
        vv_max: f32,
    ) -> Result<Transform, PixelTransformError> {
        if -vv_min != vv_max {
            Err(PixelTransformError::BadViewVolume)
        } else {
            let vv_diff = vv_max - vv_min;

            Ok(Transform {
                shift: vv_max * (pixel_extent.get() as f32 - 1.0) / vv_diff,
                scale: (pixel_extent.get() as f32) / vv_diff,
            })
        }
    }

    //flooring operation.
    fn window_to_pixel(&self, window_coord: f32) -> i32 {
        ((self.scale * window_coord).round() + self.shift) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::space::Transform;
    use std::num::NonZeroU32;

    mod pixel_transform {
        use super::{NonZeroU32, Transform};
        use crate::space::PixelTransformError::BadViewVolume;
        use crate::space::{Space, ViewVolume};

        #[test]
        fn pixel_bigger() {
            let p = Transform::new(NonZeroU32::new(100).unwrap(), -0.5, 0.5).unwrap();
            assert_eq!(p.shift, 49.5); //how many pixels to move over. Starts at 0, ends at pixel_extent - 1.
            assert_eq!(p.scale, 100.0); //the number of pixels divided by the full extent of the viewing volume
        }
        #[test]
        fn pixel_smaller() {
            let p = Transform::new(NonZeroU32::new(40).unwrap(), -50.0, 50.0).unwrap();
            assert_eq!(p.shift, 19.5); //how many pixels to move over. Starts at 0, ends at pixel_extent - 1.
            assert_eq!(p.scale, 0.4); //the number of pixels divided by the full extent of the viewing volume
        }
        // Occurrence implies a bad implementation of viewing volume, or a programming error in passing its data to PixelTransform::new().
        #[test]
        fn bad_input() {
            let p = Transform::new(NonZeroU32::new(100).unwrap(), -10.0, 11.0);
            assert_eq!(p, Err(BadViewVolume))
        }
        #[test]
        fn view_volume_width_bigger() {
            let vv = ViewVolume::new(NonZeroU32::new(100).unwrap(), NonZeroU32::new(50).unwrap());
            assert_ne!(vv.bottom, vv.left);
            assert_eq!(vv.bottom, -1.0);
            assert_eq!(vv.right, 2.0);
        }

        #[test]
        fn view_volume_height_bigger() {
            let vv = ViewVolume::new(NonZeroU32::new(40).unwrap(), NonZeroU32::new(50).unwrap());
            assert_ne!(vv.bottom, vv.left);
            assert_eq!(vv.bottom, -1.25);
            assert_eq!(vv.right, 1.0);
        }

        fn is_square(vv: ViewVolume) {
            assert_eq!(vv.top, -vv.left); //eq this time
            assert_eq!(vv.top, vv.right);
            assert_eq!(vv.top, -vv.bottom);
            assert_eq!(vv.top, 1.0);
        }

        #[test]
        fn view_volume_dimensions_same() {
            let vv = ViewVolume::new(NonZeroU32::new(100).unwrap(), NonZeroU32::new(100).unwrap());
            is_square(vv);
        }

        #[test]
        fn space_square_init() {
            let space =
                Space::new(NonZeroU32::new(100).unwrap(), NonZeroU32::new(100).unwrap()).unwrap();
            assert_eq!(space.x_transform.scale, space.y_transform.scale);
            assert_eq!(space.x_transform.shift, space.y_transform.shift);
            is_square(space.view_volume);
        }
        #[test]
        fn window_to_pixel() {
            let space =
                Space::new(NonZeroU32::new(200).unwrap(), NonZeroU32::new(100).unwrap()).unwrap();
            //assert_eq!(space.x_transform.scale, space.y_transform.scale);
            let dimension_ratio = 2.0;
            assert_eq!(
                space.x_transform.shift * 2.0 + 1.0,
                (space.y_transform.shift * 2.0 + 1.0) * dimension_ratio
            );
            let min_pic = -dimension_ratio;
            let min_vv = -1.0;
            let max_vv = 1.0;
            let max_pic = dimension_ratio;
            assert_eq!(space.x_transform.window_to_pixel(min_pic), 0);
            assert_eq!(
                space.x_transform.window_to_pixel(min_vv),
                (199f32 * 0.25).floor() as i32
            );
            assert_eq!(
                space.x_transform.window_to_pixel(max_vv),
                (199f32 * 0.75).floor() as i32
            );
            assert_eq!(space.x_transform.window_to_pixel(max_pic), 199);
        }
    }
}
