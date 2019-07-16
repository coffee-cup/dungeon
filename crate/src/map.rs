use serde::*;
use serde_derive::*;
use std::cmp;
use wasm_bindgen::prelude::*;

use crate::utils::*;
use crate::vector::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Tile {
    pub visible: bool,
    pub blocked: bool,
}

impl Tile {
    pub fn wall() -> Tile {
        Tile {
            visible: false,
            blocked: true,
        }
    }

    pub fn floor() -> Tile {
        Tile {
            visible: false,
            blocked: false,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Map {
    size: Vector,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(size: Vector) -> Map {
        let width = size.x as usize;
        let height = size.y as usize;

        let mut tiles = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    tiles.push(Tile::wall());
                } else {
                    tiles.push(Tile::floor());
                };
            }
        }

        // tiles[130] = Tile::wall();
        // tiles[131] = Tile::wall();
        // tiles[132] = Tile::wall();
        // tiles[133] = Tile::wall();
        // tiles[134] = Tile::wall();
        // tiles[135] = Tile::wall();

        Map {
            tiles: tiles,
            size: size,
        }
    }

    pub fn pos_to_index(&self, pos: Vector) -> usize {
        (self.size.x * pos.y + pos.x) as usize
    }

    pub fn is_blocked(&self, pos: Vector) -> bool {
        let index = self.pos_to_index(pos);

        if index < 0 || index >= self.tiles.len() {
            return true;
        }

        self.tiles[index].blocked
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}

pub trait ShadowCast {
    fn transparent(&self, pos: Vector) -> bool;
    fn reveal(&mut self, pos: Vector);
}

impl ShadowCast for Map {
    fn transparent(&self, pos: Vector) -> bool {
        !self.is_blocked(pos)
    }

    fn reveal(&mut self, pos: Vector) {
        let index = self.pos_to_index(pos);
        self.tiles[index].visible = true;
    }
}

#[derive(Debug)]
struct Transform {
    xx: i8,
    xy: i8,
    yx: i8,
    yy: i8,
}

impl Transform {
    pub fn new(xx: i8, xy: i8, yx: i8, yy: i8) -> Transform {
        Transform {
            xx: xx,
            xy: xy,
            yx: yx,
            yy: yy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Slope {
    pub x: i32,
    pub y: i32,
}

impl Slope {
    pub fn new(y: i32, x: i32) -> Slope {
        Slope { x: x, y: y }
    }
}

impl cmp::Ord for Slope {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self == other {
            cmp::Ordering::Equal
        } else if self.y * other.x > self.x * other.y {
            cmp::Ordering::Greater
        } else {
            cmp::Ordering::Less
        }
    }
}

impl PartialOrd for Slope {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn transform_pos(octant: usize, pos: Vector, origin: Vector) -> Vector {
    let nx = origin.x;
    let ny = origin.y;

    match octant {
        0 => Vector::new(nx + pos.x, ny - pos.y),
        1 => Vector::new(nx + pos.y, ny - pos.x),
        2 => Vector::new(nx - pos.y, ny - pos.x),
        3 => Vector::new(nx - pos.x, ny - pos.y),
        4 => Vector::new(nx - pos.x, ny + pos.y),
        5 => Vector::new(nx - pos.y, ny + pos.x),
        6 => Vector::new(nx + pos.y, ny + pos.x),
        7 => Vector::new(nx + pos.x, ny + pos.y),
        _ => panic!("not such octant!"),
    }
}

fn blocks_light(sc: &ShadowCast, pos: Vector, octant: usize, origin: Vector) -> bool {
    !sc.transparent(transform_pos(octant, pos, origin))
}

fn set_visible(sc: &mut ShadowCast, pos: Vector, octant: usize, origin: Vector) {
    sc.reveal(transform_pos(octant, pos, origin));
}

fn scan(
    sc: &mut ShadowCast,
    origin: Vector,
    rangeLimit: i32,
    x: i32,
    top: Slope,
    bottom: Slope,
    octant: usize,
) {
    let mut x = x;
    let mut top = top;
    let mut bottom = bottom;

    while rangeLimit < 0 || x <= rangeLimit {
        let topY = if top.x == 1 {
            x
        } else {
            // get the tile that the top vector enters from the left
            let mut topY = ((x * 2 - 1) * top.y + top.x) / (top.x * 2);

            // it is possible that the vector passes from the left
            // side of the tile up into the tile above before exiting
            // from the right side of this column.
            if blocks_light(sc, Vector::new(x, topY), octant, origin) {
                // the tile blocks light
                // if light passes into the tile above depends on the
                // shape of the wall tile as well as the angle of the
                // vector. if the tiel does not have a beveled
                // top-left corner, then it is blocked. the corner is
                // beveled if the tiles above and to the left are not
                // walls.
                if top >= Slope::new(topY * 2 + 1, x * 2)
                    && !blocks_light(sc, Vector::new(x, topY + 1), octant, origin)
                {
                    topY += 1;
                }
            } else {
                // the tile doesn't block light
                let mut ax = x * 2;
                if blocks_light(sc, Vector::new(x + 1, topY + 1), octant, origin) {
                    ax += 1;
                }

                if top > Slope::new(topY * 2 + 1, ax) {
                    topY += 1;
                }
            }

            topY
        };

        // get the tile that the bottom vector enters from the left.
        // ensure that the bototm vector actually hits the wall shape.
        let bottomY = if bottom.y == 0 {
            0
        } else {
            let mut bottomY = ((x * 2 - 1) * bottom.y + bottom.x) / (bottom.x * 2);

            if bottom >= Slope::new(bottomY * 2 + 1, x * 2)
                && blocks_light(sc, Vector::new(x, bottomY), octant, origin)
                && !blocks_light(sc, Vector::new(x, bottomY + 1), octant, origin)
            {
                bottomY += 1;
            }

            bottomY
        };

        // go through the tiles in the column now that we known which
        // ones could possibly be visible
        let mut was_opaque = -1; // 0:false, 1:true, -1:n/a

        let mut y = topY;
        while y >= bottomY {
            let curr = Vector::new(x, y);
            if rangeLimit < 0 || origin.distance(&curr) <= rangeLimit * 2 {
                let is_opaque = blocks_light(sc, curr, octant, origin);

                // every tile where topY > y > bottomY is guaranteed
                // to be visible. the initialization of topY and
                // bottomY guarantees that if the tile is opaque then it is visible.
                //
                // we need to do extra work if y == topY or y ==
                // bottomY. if y == topY we need to make sure that the
                // top vector is above the bottom-right corner of the
                // inner square. if y == bottomY we need to make sure
                // that the bottom vector is below the top-left corner
                // of the inner square
                let is_visible = is_opaque
                    || ((y != topY || top > Slope::new(y * 4 - 1, x * 4 + 1))
                        && (y != bottomY || bottom < Slope::new(y * 4 + 1, x * 4 - 1)));

                if is_visible {
                    set_visible(sc, curr, octant, origin)
                };

                // if we found a transition from clear to opaque or
                // vice versa, adjust the otp and bottom vectors
                // but, don't bother adjusting them if this is the last column
                if x != rangeLimit {
                    if is_opaque {
                        if was_opaque == 0 {
                            // top center
                            let mut nx = x * 2;
                            let ny = y * 2 + 1;

                            if blocks_light(sc, Vector::new(x, y), octant, origin) {
                                nx -= 1;
                            }

                            if top > Slope::new(ny, nx) {
                                if y == bottomY {
                                    bottom = Slope::new(ny, nx);
                                } else {
                                    scan(
                                        sc,
                                        origin,
                                        rangeLimit,
                                        x + 1,
                                        top,
                                        Slope::new(ny, nx),
                                        octant,
                                    );
                                }
                            } else {
                                if y == bottomY {
                                    return;
                                }
                            }
                        }
                    }
                } else {
                    // found transition from opaque to clear, adjust the top vector downwards
                    if was_opaque > 0 {
                        // bottom of the opaque tile
                        let mut nx = x * 2;
                        let ny = y * 2 + 1;

                        if blocks_light(sc, Vector::new(x + 1, y + 1), octant, origin) {
                            nx += 1
                        }

                        if bottom >= Slope::new(ny, nx) {
                            return;
                        } else {
                            top = Slope::new(ny, nx);
                        }
                    }

                    was_opaque = 0;
                }
            }

            y -= 1;
        }

        if was_opaque != 0 {
            break;
        }

        x += 1;
    }
}

pub fn shadowcast(sc: &mut ShadowCast, origin: Vector) {
    sc.reveal(origin);

    for i in 0..8 {
        scan(sc, origin, -1, 1, Slope::new(1, 1), Slope::new(0, 1), i);
    }
}
// type Slope = f32;

// fn scan(
//     sc: &mut ShadowCast,
//     origin: Vector,
//     y: f32,
//     start: Slope,
//     end: Slope,
//     transform: Transform,
// ) {
//     let cx = origin.x as f32;
//     let cy = origin.y as f32;

//     if start >= end {
//         return;
//     }

//     let xmin = ((y - 0.5) * start).floor() as i32;
//     let xmax = ((y + 0.5) * end - 0.5).ceil() as i32;

//     let mut new_start = start;

//     for x in xmin..xmax {
//         let x = x as f32;
//         let realx = cx + (transform.xx as f32) * cx + (transform.xy as f32) * y;
//         let realy = cy + (transform.yx as f32) * cx + (transform.yy as f32) * y;

//         let real = Vector::new(realx as i32, realy as i32);

//         if sc.transparent(real) {
//             if x >= y * new_start && x <= y * end {
//                 sc.reveal(real);
//             }
//         } else {
//             if x >= (y - 0.5) * new_start && x - 0.5 <= y * end {
//                 // why?
//                 sc.reveal(real);
//             }

//             scan(sc, origin, y + 1.0, new_start, (x - 0.5) / y, transform);

//             new_start = (x + 0.5) / y;

//             if new_start >= end {
//                 return;
//             }
//         }
//     }

//     scan(sc, origin, y + 1.0, new_start, end, transform);
// }

// pub fn shadowcast(sc: &mut ShadowCast, origin: Vector) {
//     sc.reveal(origin);
//     console_log!("test");

//     let transforms = [
//         Transform::new(1, 0, 0, 1),
//         Transform::new(1, 0, 0, -1),
//         Transform::new(-1, 0, 0, -1),
//         Transform::new(-1, 0, 0, -1),
//         Transform::new(0, 1, 1, 0),
//         Transform::new(0, 1, -1, 1),
//         Transform::new(0, -1, 1, 0),
//         Transform::new(0, -1, -1, 0),
//     ];

//     for i in 0..transforms.len() {
//         scan(sc, origin, 1.0, 0.0, 1.0, transforms[i]);
//     }
// }
