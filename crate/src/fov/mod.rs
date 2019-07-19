use crate::vector::*;

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

    pub fn project(row: i32, col: i32) -> Shadow {
        let row = row as f32;
        let col = col as f32;

        let top_left = col / (row + 2.0);
        let bottom_right = (col + 1.0) / (row + 1.0);

        Shadow::new(top_left, bottom_right)
    }

    pub fn contains(&self, other: &Shadow) -> bool {
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
    while true {
        if !area.in_bounds(origin + transform_octant(row, 0, octant)) {
            break;
        }

        let mut col = 0;
        while col <= row {
            let pos = origin + transform_octant(row, col, octant);
            if !area.in_bounds(pos) {
                break;
            }

            let out_of_range = range_limit >= 0 && !origin.in_range(&pos, range_limit);

            if full_shadow || out_of_range {
                area.set_visibility(pos, false);
            } else {
                let projection = Shadow::project(row, col);

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
