// using the crates prelude grants access to bracket-lib
// and everything else in the main.rs prelude.
use crate::prelude::*;

// constants can only be calculated from other constants.
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
// usize uses the prefered size for your CPU.
// a 64 bit computer will be 64 bits.
// Rust commonly uses usize to index collections and arrays.

#[derive(Copy, Clone, PartialEq)]
// The above derivations are:
// Copy - changes the default action when assigning a TileType
// from one variable to another. Instead of moving the value, it
// takes a copy. Smaller types are often faster when you copy them
// around.
// Clone - adds a clone() method, calling my_tile.clone() will
// make a deep copy of my_tile without affecting the original.
// PartialEq - adds code that allows you to compare two TileType
// values with the == operator.
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    // The constructor for Map
    // create a map consisting entirely of floor tiles.
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
    // the render function uses a nested loop to iterate through all x and y values
    // iterating y first is faster with row-first striding due to memory cache usage.
    // match determines the tile type and set draws the tile.
    // floor appears as a yellow . and wall appears as a green #.
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }
}

// Vectors are indexed on a single dimension, so need to
// transform x,y to the vector index
// The vector will use row-encoding, where each row of the
// map is stored together in x order.
// calculating vector index from x and y, if y is 3, this
// means index is at least three complete rows deep in the vector.
// so need to advance the index to the fourth row. If x is 2, this
// means the index is the second tile in the fourth row. This can
// be simplified to
// let index = (y * WIDTH) + x;
// the x and y can be calculated from the index with
// let x = index % WIDTH;
// let y = index / WIDTH; // dividing integers always rounds down.

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
