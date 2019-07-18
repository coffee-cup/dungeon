use std::cmp;

use crate::vector::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Slope {
    pub x: i32,
    pub y: i32,
}

impl Slope {
    pub fn new(y: i32, x: i32) -> Slope {
        Slope { x: x, y: y }
    }

    pub fn to_num(&self) -> i32 {
        self.y / self.x
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

#[derive(Debug, Clone)]
struct Shadow {
    pub start: Slope,
    pub end: Slope,
}

impl Shadow {
    pub fn new(start: Slope, end: Slope) -> Shadow {
        Shadow {
            start: start,
            end: end,
        }
    }

    pub fn project(pos: Vector) -> Shadow {
        // y / x ?
        let top_left = Slope::new(pos.x, pos.y + 2);
        let bottom_right = Slope::new(pos.x + 1, pos.y + 1);
        Shadow::new(top_left, bottom_right)
    }

    pub fn contains(&self, other: &Shadow) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

#[derive(Debug)]
struct ShadowLine {
    shadows: Vec<Shadow>,
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
        for (i, s) in self.shadows.iter().enumerate() {
            if s.start >= shadow.start {
                index = i;
                break;
            }
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
        self.shadows.len() == 1
            && self.shadows[0].start.to_num() == 0
            && self.shadows[0].end.to_num() == 1
    }
}

pub trait Area {
    fn transparent(&self, pos: Vector) -> bool;
    fn in_bounds(&self, pos: Vector) -> bool;
    fn set_visibility(&mut self, pos: Vector, visibility: bool);
}

fn transform_octant(octant: usize, pos: Vector) -> Vector {
    match octant {
        0 => Vector::new(pos.x, -pos.y),
        1 => Vector::new(pos.y, -pos.x),
        2 => Vector::new(pos.y, pos.x),
        3 => Vector::new(pos.x, pos.y),
        4 => Vector::new(-pos.x, pos.y),
        5 => Vector::new(-pos.y, pos.x),
        6 => Vector::new(-pos.y, -pos.x),
        7 => Vector::new(-pos.x, -pos.y),
        _ => panic!("not such octant!"),
    }
}

pub fn refresh_visiblity(area: &mut Area, range_limit: i32, origin: Vector) {
    for octant in 0..8 {
        refresh_octant(area, range_limit, origin, octant);
    }
}

fn refresh_octant(area: &mut Area, range_limit: i32, origin: Vector, octant: usize) {
    let mut line = ShadowLine::new();
    let mut full_shadow = false;

    let mut row = 0;
    while range_limit < 0 || row < range_limit {
        let pos = origin + transform_octant(octant, Vector::new(row, 0));
        if !area.in_bounds(pos) {
            break;
        }

        let mut col = 0;
        while col <= row {
            let pos = origin + transform_octant(octant, Vector::new(row, col));
            if !area.in_bounds(pos) {
                break;
            }

            if full_shadow {
                area.set_visibility(pos, false);
            } else {
                let projection = Shadow::project(pos);

                let visible = !line.is_in_shadow(&projection);
                area.set_visibility(pos, visible);

                // add any opaque itles to the shadow map
                if visible && !area.transparent(pos) {
                    line.add(projection.clone());
                    full_shadow = line.is_full_shadow();
                }
            }

            col += 1;
        }

        row += 1;
    }
}
