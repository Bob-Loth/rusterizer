use std::ops::RangeInclusive;

use crate::io::Mode;
use crate::{space, Fragment};
use array2d::Array2D;
use barycentric::BaryCentricConstants;
use space::Space;

mod barycentric {
    use crate::point::{Fragment, Triangle};

    //       c_______b
    //		 \  pav /
    //     pac\  | /pab
    //         \ |/
    //			\/a
    //
    #[derive(Debug)]
    pub(crate) struct BaryCentricConstants {
        pub(crate) pab: Fragment,
        pub(crate) pac: Fragment,
        pub(crate) pabac: i64,
        pub(crate) total_area: i64,
    }
    impl From<&Triangle> for BaryCentricConstants {
        fn from(tri: &Triangle) -> Self {
            let pab = tri.b - tri.a;
            let pac = tri.c - tri.a;
            let pabac = pab.dot(pac);
            let total_area = (pab.dot_self()) * (pac.dot_self()) - pabac * pabac;
            BaryCentricConstants {
                pab,
                pac,
                pabac,
                total_area,
            }
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[derive(Debug)]
struct Triangle {
    a: Fragment,
    b: Fragment,
    c: Fragment,
    bounding_box: BoundingBox,
    barycentric_constants: BaryCentricConstants,
}

#[derive(Debug)]
pub(crate) struct BarycentricResult {
    z: f32,
    alpha: f32,
    beta: f32,
    gamma: f32,
}

impl Triangle {
    fn new(space: &Space, w_a: Point, w_b: Point, w_c: Point) -> Self {
        let a = space.window_to_pixel(w_a);
        let b = space.window_to_pixel(w_b);
        let c = space.window_to_pixel(w_c);
        Triangle {
            a,
            b,
            c,
            bounding_box: BoundingBox {
                x_min: a.x.min(b.x).min(c.x),
                x_max: a.x.max(b.x).max(c.x),
                y_min: a.y.min(b.y).min(c.y),
                y_max: a.y.max(b.y).max(c.y),
            },
            barycentric_constants: {
                let pab = b - a;
                let pac = c - a;
                let pabac = pab.dot(pac);

                let total_area = (pab.dot_self()) * (pac.dot_self()) - (pabac * pabac);
                BaryCentricConstants {
                    pab,
                    pac,
                    pabac,
                    total_area,
                }
            },
        }
    }
    pub(crate) fn barycentric_coordinates(&self, v: &Fragment) -> BarycentricResult {
        let pav = *v - self.a; //vector from triangle's "a" to a given fragment v
        let pavab: i64 = pav.dot(self.barycentric_constants.pab);

        //(ac * ac) * (av * ab) - (ab * ac) * (av * ac), with floating-point divide
        let beta = (self.barycentric_constants.pac.dot_self() * pavab
            - (self.barycentric_constants.pabac * pav.dot(self.barycentric_constants.pac)))
            as f32
            / self.barycentric_constants.total_area as f32;
        //(ab * ab) * (av * ac) - (ab * ac) * (av * ab), with floating-point divide
        let gamma = (self.barycentric_constants.pab.dot_self()
            * pav.dot(self.barycentric_constants.pac)
            - self.barycentric_constants.pabac * pavab) as f32
            / self.barycentric_constants.total_area as f32;
        //from a + b + c = 1
        let alpha = 1.0 - beta - gamma;
        let z = alpha * self.a.z + beta * self.b.z + gamma * self.c.z;
        BarycentricResult {
            z,
            alpha,
            beta,
            gamma,
        }
    }
}
#[derive(Debug)]
struct BoundingBox {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl BoundingBox {
    //row, un-intuitively uses the y coordinate because it is a bound of a particular range of y-coordinates.
    //      x=1 x=2 x=3
    //y = 1  A   B   C      range of y = 1..3 covers the first two rows.
    //y = 2  D   E   F
    //y = 3  G   H   I
    pub(crate) fn row_range(&self) -> RangeInclusive<i64> {
        self.y_min..=self.y_max
    }
    //vice versa for columns and x coordinates
    pub(crate) fn column_range(&self) -> RangeInclusive<i64> {
        self.x_min..=self.x_max
    }
}

pub fn rasterize(
    pixels: &mut Array2D<f32>,
    space: &Space,
    vertices: &[f32],
    indices: &[u32],
    mode: Mode,
) {
    assert_eq!(indices.len() % 3, 0);
    //for every triangle with coords x,y,z

    for triangle in indices
        .chunks_exact(3) //collect 3 indices.
        .map(|index| {
            let a = Point {
                //construct a Point for each index from slice. 0,1,2 indexed from beginning of slice 0,3,6...
                x: vertices[(index[0] * 3) as usize],
                y: vertices[(index[0] * 3 + 1) as usize],
                z: vertices[(index[0] * 3 + 2) as usize],
            };
            let b = Point {
                x: vertices[(index[1] * 3) as usize],
                y: vertices[(index[1] * 3 + 1) as usize],
                z: vertices[(index[1] * 3 + 2) as usize],
            };
            let c = Point {
                x: vertices[(index[2] * 3) as usize],
                y: vertices[(index[2] * 3 + 1) as usize],
                z: vertices[(index[2] * 3 + 2) as usize],
            };
            //also computes bounding box and constant factors of barycentric coordinate evaluation
            Triangle::new(space, a, b, c)
        })
    {
        //iterate over every pixel in the bounding box
        write_triangle(pixels, mode, triangle);
    }
}

fn write_triangle(pixels: &mut Array2D<f32>, mode: Mode, triangle: Triangle) {
    for row_idx in triangle.bounding_box.row_range() {
        for column_idx in triangle.bounding_box.column_range() {
            //compute barycentric coordinates, returning an alpha, beta, and gamma value.
            write_pixel(pixels, mode, &triangle, row_idx, column_idx);
        }
    }
}

fn write_pixel(
    pixels: &mut Array2D<f32>,
    mode: Mode,
    triangle: &Triangle,
    row_idx: i64,
    column_idx: i64,
) {
    let mut frag = Fragment {
        x: row_idx,
        y: column_idx,
        z: 0.0,
    };
    let bary = triangle.barycentric_coordinates(&frag);
    frag.z = bary.z; //explicit about where assignment is happening
                     //if Point is inside triangle,
    if inside_triangle(bary.alpha, bary.beta, bary.gamma) {
        //perform wireframe or depth coloring
        match mode {
            Mode::Depth => color_depth(pixels, frag),
            Mode::Wireframe => {
                const EPSILON: f32 = 0.3;
                if [bary.alpha, bary.beta, bary.gamma]
                    .iter()
                    .all(|&b| f32::abs(b) < EPSILON)
                {
                    color_depth(pixels, frag);
                }
            }
        }
    }
}

//       |  /   .zbuf (1)
//       | /   /
//       |/   .cur (.33)
// ------|------
//      /|
//     / |
//    /  |
fn color_depth(pixels: &mut Array2D<f32>, frag: Fragment) {
    if let Some(pixel) = pixels.get_mut(frag.x as usize, frag.y as usize) {
        *pixel = pixel.min(frag.z);
    }
}

fn inside_triangle(alpha: f32, beta: f32, gamma: f32) -> bool {
    let range = 0f32..=1f32;
    range.contains(&alpha) & range.contains(&beta) & range.contains(&gamma)
}

#[cfg(test)]
mod tests {
    use crate::point::{inside_triangle, Point, Triangle};
    use crate::space::Space;
    use crate::Fragment;
    use std::num::NonZeroU32;

    #[test]
    fn triangle_creation() {
        let space = Space::new(NonZeroU32::new(10).unwrap(), NonZeroU32::new(10).unwrap());
        let tri = Triangle::new(
            &space.unwrap(),
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point {
                x: 0.0,
                y: 10.0,
                z: 0.0,
            },
            Point {
                x: 10.0,
                y: 0.0,
                z: 0.0,
            },
        );
        let frags = vec![
            Fragment { x: 5, y: 5, z: 0.0 }, //good
            Fragment { x: 2, y: 3, z: 0.0 }, //bad
            Fragment { x: 0, y: 0, z: 0.0 }, //bad
            Fragment { x: 5, y: 6, z: 0.0 }, //good
            Fragment { x: 6, y: 5, z: 0.0 }, //good
        ];

        let mut results: Vec<bool> = vec![];
        for frag in &frags {
            let bary = tri.barycentric_coordinates(frag);
            println!("{:?}", bary);
            results.push(inside_triangle(bary.alpha, bary.beta, bary.gamma));
        }
        assert_eq!(results, vec![true, false, false, true, true]);
    }
}
