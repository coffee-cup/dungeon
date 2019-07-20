use rand::prelude::*;

pub fn percent() -> u32 {
    let mut rng = thread_rng();
    let p: u32 = rng.gen_range(0, 100);
    p
}
