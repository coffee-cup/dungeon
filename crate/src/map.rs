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

        let mut map = Map {
            tiles: tiles,
            size: size,
        };

        let walls: Vec<Vector> = vec![
            Vector::new(4, 2),
            Vector::new(5, 2),
            Vector::new(6, 2),
            Vector::new(7, 2),
            Vector::new(8, 2),
            Vector::new(9, 2),
            Vector::new(10, 2),
            Vector::new(11, 2),
            Vector::new(12, 2),
            Vector::new(13, 2),
            Vector::new(4, 13),
            Vector::new(4, 14),
            Vector::new(4, 15),
            Vector::new(5, 15),
            Vector::new(5, 14),
            Vector::new(6, 15),
        ];

        for v in walls.iter() {
            let index = map.pos_to_index(*v);
            map.tiles[index] = Tile::wall();
        }

        map
    }

    pub fn hide_all(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.visible = false;
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
        if (pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y) {
            return;
        }

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
    let new_pos = transform_pos(octant, pos, origin);
    // console_log!("setting visible {},{}", new_pos.x, new_pos.y);
    sc.reveal(transform_pos(octant, pos, origin));
}

fn scan(
    sc: &mut ShadowCast,
    origin: Vector,
    range_limit: i32,
    x: i32,
    top: Slope,
    bottom: Slope,
    octant: usize,
) {
    let mut x = x;
    let mut top = top;
    let mut bottom = bottom;

    while range_limit < 0 || x <= range_limit {
        let top_y = if top.x == 1 {
            x
        } else {
            // get the tile that the top vector enters from the left
            let mut top_y = ((x * 2 - 1) * top.y + top.x) / (top.x * 2);

            // it is possible that the vector passes from the left
            // side of the tile up into the tile above before exiting
            // from the right side of this column.
            if blocks_light(sc, Vector::new(x, top_y), octant, origin) {
                // the tile blocks light
                // if light passes into the tile above depends on the
                // shape of the wall tile as well as the angle of the
                // vector. if the tiel does not have a beveled
                // top-left corner, then it is blocked. the corner is
                // beveled if the tiles above and to the left are not
                // walls.
                if top >= Slope::new(top_y * 2 + 1, x * 2)
                    && !blocks_light(sc, Vector::new(x, top_y + 1), octant, origin)
                {
                    top_y += 1;
                }
            } else {
                // the tile doesn't block light
                let mut ax = x * 2;
                if blocks_light(sc, Vector::new(x + 1, top_y + 1), octant, origin) {
                    ax += 1;
                }

                if top > Slope::new(top_y * 2 + 1, ax) {
                    top_y += 1;
                }
            }

            top_y
        };

        // get the tile that the bottom vector enters from the left.
        // ensure that the bototm vector actually hits the wall shape.
        let bottom_y = if bottom.y == 0 {
            0
        } else {
            let mut bottom_y = ((x * 2 - 1) * bottom.y + bottom.x) / (bottom.x * 2);

            if bottom >= Slope::new(bottom_y * 2 + 1, x * 2)
                && blocks_light(sc, Vector::new(x, bottom_y), octant, origin)
                && !blocks_light(sc, Vector::new(x, bottom_y + 1), octant, origin)
            {
                bottom_y += 1;
            }

            bottom_y
        };

        // go through the tiles in the column now that we known which
        // ones could possibly be visible
        let mut was_opaque = -1; // 0:false, 1:true, -1:n/a

        let mut y = top_y;
        while y >= bottom_y {
            let curr = Vector::new(x, y);
            if range_limit < 0 || origin.distance(&curr) <= range_limit * 2 {
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
                    || ((y != top_y || top > Slope::new(y * 4 - 1, x * 4 + 1))
                        && (y != bottom_y || bottom < Slope::new(y * 4 + 1, x * 4 - 1)));

                if is_visible {
                    set_visible(sc, curr, octant, origin)
                };

                // if we found a transition from clear to opaque or
                // vice versa, adjust the otp and bottom vectors
                // but, don't bother adjusting them if this is the last column
                if x != range_limit {
                    if is_opaque {
                        if was_opaque == 0 {
                            // top center
                            let mut nx = x * 2;
                            let ny = y * 2 + 1;

                            if blocks_light(sc, Vector::new(x, y), octant, origin) {
                                nx -= 1;
                            }

                            if top > Slope::new(ny, nx) {
                                if y == bottom_y {
                                    bottom = Slope::new(ny, nx);
                                } else {
                                    scan(
                                        sc,
                                        origin,
                                        range_limit,
                                        x + 1,
                                        top,
                                        Slope::new(ny, nx),
                                        octant,
                                    );
                                }
                            } else {
                                if y == bottom_y {
                                    return;
                                }
                            }

                            was_opaque = 1;
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

    let range_limit = -1;

    // scan(
    //     sc,
    //     origin,
    //     range_limit,
    //     1,
    //     Slope::new(1, 1),
    //     Slope::new(0, 1),
    //     0,
    // );

    for i in 0..8 {
        scan(
            sc,
            origin,
            range_limit,
            1,
            Slope::new(1, 1),
            Slope::new(0, 1),
            i,
        );
    }
}
