use serde::*;
use serde_derive::*;
use std::cmp;
use wasm_bindgen::prelude::*;

use crate::vector::*;

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

        tiles[130] = Tile::wall();
        tiles[131] = Tile::wall();
        tiles[132] = Tile::wall();
        tiles[133] = Tile::wall();
        tiles[134] = Tile::wall();
        tiles[135] = Tile::wall();

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

#[derive(Debug, Clone, Copy)]
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

type Slope = f32;

fn scan(
    sc: &mut ShadowCast,
    origin: Vector,
    y: f32,
    start: Slope,
    end: Slope,
    transform: Transform,
) {
    let cx = origin.x as f32;
    let cy = origin.y as f32;

    if start >= end {
        return;
    }

    let xmin = ((y - 0.5) * start).floor() as i32;
    let xmax = ((y + 0.5) * end - 0.5).ceil() as i32;

    let mut new_start = start;

    for x in xmin..xmax {
        let x = x as f32;
        let realx = cx + (transform.xx as f32) * cx + (transform.xy as f32) * y;
        let realy = cy + (transform.yx as f32) * cx + (transform.yy as f32) * y;

        let real = Vector::new(realx as i32, realy as i32);

        if sc.transparent(real) {
            if x >= y * new_start && x <= y * end {
                sc.reveal(real);
            }
        } else {
            if x >= (y - 0.5) * new_start && x - 0.5 <= y * end {
                sc.reveal(real);
            }

            scan(sc, origin, y + 1.0, new_start, (x - 0.5) / y, transform);

            new_start = (x + 0.5) / y;

            if new_start >= end {
                return;
            }
        }
    }

    scan(sc, origin, y + 1.0, new_start, end, transform);
}

pub fn shadowcast(sc: &mut ShadowCast, origin: Vector) {
    sc.reveal(origin);

    let transforms = [
        Transform::new(1, 0, 0, 1),
        Transform::new(1, 0, 0, -1),
        Transform::new(-1, 0, 0, -1),
        Transform::new(-1, 0, 0, -1),
        Transform::new(0, 1, 1, 0),
        Transform::new(0, 1, -1, 1),
        Transform::new(0, -1, 1, 0),
        Transform::new(0, -1, -1, 0),
    ];

    for i in 0..transforms.len() {
        scan(sc, origin, 1.0, 0.0, 1.0, transforms[i]);
    }
}
