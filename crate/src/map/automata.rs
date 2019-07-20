use crate::map::tile::*;
use crate::random;
use crate::vector::*;

pub fn generate_automata_map(size: Vector) -> Vec<Tile> {
    random_tiles(size)
}

fn random_tiles(size: Vector) -> Vec<Tile> {
    let width = size.x as usize;
    let height = size.y as usize;

    let mut tiles = Vec::with_capacity(width * height);

    let wall_chance = 45;
    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                tiles.push(Tile::wall());
            } else {
                let r = random::percent();
                if r < wall_chance {
                    tiles.push(Tile::wall());
                } else {
                    tiles.push(Tile::floor());
                }
            };
        }
    }

    tiles
}
