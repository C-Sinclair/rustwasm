mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize 
    }
    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_column in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_column = (column + delta_column) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_column);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);
                let next_cell = match (cell, live_neighbours) {
                    // Rule 1, underpopulation causes death of cell 
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2, live if 2/3 neighbours
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3, overpopulation causes death of cell
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 4, dead cell with 3 live neighbours is born
                    (Cell::Dead, 3) => Cell::Alive
                    // All other cells remain the same
                    (otherwise, _) => otherwise
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
}