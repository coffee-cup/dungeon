use std::cmp;
use wasm_bindgen::prelude::*;

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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Slope {
    x: i32,
    y: i32,
    pub num: f32,
}

impl Slope {
    pub fn new(y: i32, x: i32) -> Slope {
        Slope {
            x: x,
            y: y,
            num: (y as f32) / (x as f32),
        }
    }

    pub fn to_num(&self) -> i32 {
        self.y / self.x
    }
}

// impl cmp::Ord for Slope {
//     fn cmp(&self, other: &Self) -> cmp::Ordering {
//         if self == other {
//             cmp::Ordering::Equal
//         } else if self.y * other.x > self.x * other.y {
//             cmp::Ordering::Greater
//         } else {
//             cmp::Ordering::Less
//         }
//     }
// }

// impl PartialOrd for Slope {
//     fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

#[derive(Debug, Clone)]
struct Shadow {
    pub start: f32,
    pub end: f32,
}

impl Shadow {
    pub fn new(start: f32, end: f32) -> Shadow {
        Shadow {
            start: start,
            end: end,
        }
    }
    // pub fn new(start: Slope, end: Slope) -> Shadow {
    //     Shadow {
    //         start: start,
    //         end: end,
    //     }
    // }

    pub fn project(row: i32, col: i32) -> Shadow {
        // y / x ?
        // let top_left = Slope::new(pos.x, pos.y + 2);
        // let bottom_right = Slope::new(pos.x + 1, pos.y + 1);
        let row = row as f32;
        let col = col as f32;

        let top_left = col / (row + 2.0);
        let bottom_right = (col + 1.0) / (row + 1.0);

        Shadow::new(top_left, bottom_right)
    }

    pub fn contains(&self, other: &Shadow) -> bool {
        // self.start.num <= other.start.num && self.end.num >= other.end.num
        self.start <= other.start && self.end >= other.end
    }
}

#[derive(Debug)]
struct ShadowLine {
    pub shadows: Vec<Shadow>,
}

impl ShadowLine {
    pub fn new() -> ShadowLine {
        ShadowLine {
            shadows: Vec::new(),
        }
    }

    pub fn is_in_shadow(&self, projection: &Shadow) -> bool {
        for shadow in self.shadows.iter() {
            if shadow.contains(&projection) {
                return true;
            }
        }

        false
    }

    pub fn add(&mut self, shadow: Shadow) {
        // find where the new shadow fits
        let mut index = 0;

        while index < self.shadows.len() {
            if self.shadows[index].start >= shadow.start {
                break;
            }
            index += 1;
        }

        // see if new shadow overlaps with previous or next
        let mut overlaps_prev = false;
        let mut overlaps_next = false;

        if index > 0 && self.shadows[index - 1].end > shadow.start {
            overlaps_prev = true;
        }

        if index < self.shadows.len() && self.shadows[index].start < shadow.end {
            overlaps_next = true;
        }

        // insert and unify with overlapping shadows
        if overlaps_next {
            if overlaps_prev {
                // overlaps both, unify one and delete the other
                self.shadows[index - 1].end = self.shadows[index].end;
                self.shadows.remove(index);
            } else {
                // overlaps next
                self.shadows[index].start = shadow.start;
            }
        } else {
            if overlaps_prev {
                // overlaps prev
                self.shadows[index - 1].end = shadow.end;
            } else {
                // does not overlap anything
                self.shadows.insert(index, shadow);
            }
        }
    }

    pub fn is_full_shadow(&self) -> bool {
        self.shadows.len() == 1 && self.shadows[0].start == 0.0 && self.shadows[0].end == 1.0
    }
}

pub trait Area {
    fn transparent(&self, pos: Vector) -> bool;
    fn in_bounds(&self, pos: Vector) -> bool;
    fn set_visibility(&mut self, pos: Vector, visibility: bool);
}

fn transform_octant(row: i32, col: i32, octant: usize) -> Vector {
    match octant {
        0 => Vector::new(col, -row),
        1 => Vector::new(row, -col),
        2 => Vector::new(row, col),
        3 => Vector::new(col, row),
        4 => Vector::new(-col, row),
        5 => Vector::new(-row, col),
        6 => Vector::new(-row, -col),
        7 => Vector::new(-col, -row),
        _ => panic!("not such octant!"),
    }
}

pub fn refresh_visiblity(area: &mut Area, range_limit: i32, origin: Vector) {
    refresh_octant(area, range_limit, origin, 0);

    for octant in 0..8 {
        refresh_octant(area, range_limit, origin, octant);
    }
}

fn refresh_octant(area: &mut Area, range_limit: i32, origin: Vector, octant: usize) {
    let mut line = ShadowLine::new();
    let mut full_shadow = false;

    let mut row = 1;
    while range_limit < 0 || row < range_limit {
        if !area.in_bounds(origin + transform_octant(row, 0, octant)) {
            break;
        }

        let mut col = 0;
        while col <= row {
            let pos = origin + transform_octant(row, col, octant);
            if !area.in_bounds(pos) {
                break;
            }

            if full_shadow {
                area.set_visibility(pos, false);
            } else {
                let projection = Shadow::project(row, col);

                let visible = !line.is_in_shadow(&projection);
                area.set_visibility(pos, visible);

                // add any opaque itles to the shadow map
                if visible && !area.transparent(pos) {
                    line.add(projection.clone());
                    // full_shadow = line.is_full_shadow();
                }
            }

            // console_log!("-------------");
            // console_log!("looking at {}", pos);
            // console_log!("{:?}", line);

            col += 1;
        }

        row += 1;
    }
}
