use serde::*;
use serde_derive::*;
use serde_repr::*;
use wasm_bindgen::prelude::*;

use crate::fov;
use crate::map::*;
use crate::random;
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

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy, Serialize_repr)]
#[repr(u8)]
pub enum Direction {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize)]
pub struct Game {
    map: Map,
    pub player: Vector,
    pub range_limit: i32,
    pub size: Vector,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen]
    pub fn new(width: i32, height: i32, map_str: &str) -> Game {
        let mut map = Map::new(Vector::new(width, height), map_str);

        console_log!("{}", map);

        let player = find_player_start(&map);
        let range_limit = -1;

        map.set_all_visiblity(true);
        fov::refresh_visiblity(&mut map, range_limit, player);

        Game {
            map: map,
            player: player,
            range_limit: range_limit,
            size: Vector::new(width, height),
        }
    }

    #[wasm_bindgen]
    pub fn get_map(&self) -> JsValue {
        JsValue::from_serde(&self.map).unwrap()
    }

    #[wasm_bindgen]
    pub fn move_player(&mut self, dir: Direction) {
        let player = self.player;

        let new_pos = match dir {
            Direction::N => Vector {
                y: player.y - 1,
                ..player
            },
            Direction::E => Vector {
                x: player.x + 1,
                ..player
            },
            Direction::S => Vector {
                y: player.y + 1,
                ..player
            },
            Direction::W => Vector {
                x: player.x - 1,
                ..player
            },
            Direction::NE => Vector {
                x: player.x + 1,
                y: player.y - 1,
                ..player
            },
            Direction::SE => Vector {
                x: player.x + 1,
                y: player.y + 1,
                ..player
            },
            Direction::NW => Vector {
                x: player.x - 1,
                y: player.y - 1,
                ..player
            },
            Direction::SW => Vector {
                x: player.x - 1,
                y: player.y + 1,
                ..player
            },
            _ => Vector { ..player },
        };

        if !self.map.is_blocked(new_pos) {
            self.player = new_pos;
        }

        self.map.set_all_visiblity(true);
        fov::refresh_visiblity(&mut self.map, self.range_limit, self.player);
    }
}

fn find_player_start(map: &Map) -> Vector {
    let mut pos = Vector::new(0, 0);
    loop {
        let rand_x = random::range(0, map.width as u32) as i32;
        let rand_y = random::range(0, map.height as u32) as i32;

        let new_pos = Vector::new(rand_x, rand_y);
        if !map.is_blocked(new_pos) {
            pos = new_pos;
            break;
        }
    }

    pos
}
