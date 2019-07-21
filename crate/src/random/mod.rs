use rand::prelude::*;

pub fn percent() -> u32 {
    range(0, 100)
}

pub fn range(start: u32, end: u32) -> u32 {
    thread_rng().gen_range(start, end)
}
