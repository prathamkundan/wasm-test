mod utils;
extern crate web_sys;

use std::fmt::Display;

use wasm_bindgen::prelude::*;
use web_sys::console;

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
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count: u8 = 0;
        for d_r in -1..=1 {
            for d_c in -1..=1 {
                if d_r == 0 && d_c == 0 {
                    continue;
                }
                let n_r: u32 = (row as i32 + d_r + self.height as i32) as u32 % self.height;
                let n_c: u32 = (column as i32 + d_c + self.width as i32) as u32 % self.width;
                count += self.cells[self.get_index(n_r, n_c)] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..(self.width * self.height))
            .map(|_| Cell::Dead)
            .collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..(self.width * self.height))
            .map(|_| Cell::Dead)
            .collect();
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) -> () {
        let _timer = Timer::new("universe_tick");
        let mut next_gen = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let idx: usize = self.get_index(row, column);
                let cell: Cell = self.cells[idx];
                let live_neighbors: u8 = self.live_neighbor_count(row, column);

                next_gen[idx] = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                }
            }
        }

        self.cells = next_gen;
    }

    pub fn new() -> Universe {
        let width = 128;
        let height = 128;

        let cells = (0..width * height)
            .map(|x| {
                if x % 2 == 0 || x % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32){
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Alive { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub struct Timer<'a> {
    name: &'a str
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a>{
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}