use serde::*;
use serde_derive::*;
use serde_repr::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy, Serialize_repr)]
#[repr(u8)]
pub enum EntityType {
    Player = 0,
    Wall = 1,
    Floor = 2,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Vector {
        Vector { x: x, y: y }
    }
}

pub type Map = Vec<EntityType>;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Game {
    map: Map,
    pub size: Vector,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen]
    pub fn new(width: u32, height: u32) -> Game {
        let map = generate_map(Vector::new(width as i32, height as i32));

        Game {
            map: map,
            size: Vector::new(width as i32, height as i32),
        }
    }

    #[wasm_bindgen]
    pub fn get_map(&self) -> JsValue {
        let map = self.map.clone();

        JsValue::from_serde(&map).unwrap()
    }
}

fn generate_map(size: Vector) -> Map {
    let width = size.x as usize;
    let height = size.y as usize;

    let mut map = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                map.push(EntityType::Wall)
            } else {
                map.push(EntityType::Floor);
            };
        }
    }

    map
}
