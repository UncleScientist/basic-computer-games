use std::collections::{HashMap, hash_map::Entry};

use macroquad::prelude::*;

pub struct Grid {
    padding: f32, // padding around the outside of the grid
    columns: f32,
    rows: f32,
    chars: HashMap<char, (String, TextDimensions)>,
    grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(padding: f32, columns: usize, rows: usize) -> Self {
        Self {
            padding,
            columns: columns as f32,
            rows: rows as f32,
            chars: HashMap::new(),
            grid: Vec::new(),
        }
    }

    pub fn set_padding(&mut self, padding: f32) {
        self.padding = padding;
    }

    pub fn draw(&self) {
        let w = screen_width();
        let h = screen_height();

        // Calculate full width and height minus the surrounding padding
        // We put 2x padding on each side
        let grid_width = w - self.padding * 4.0;
        let grid_height = h - self.padding * 4.0;

        let mut vertical_pos = self.padding * 2.0;
        while vertical_pos < w - self.padding * 2.0 + 1.0 {
            draw_line(
                vertical_pos,
                self.padding * 2.0,
                vertical_pos,
                grid_height + self.padding * 2.0,
                2.0,
                GRAY,
            );
            vertical_pos += grid_width / self.columns;
        }

        let mut horizontal_pos = self.padding * 2.0;
        while horizontal_pos < h - self.padding * 2.0 + 1.0 {
            draw_line(
                self.padding * 2.0,
                horizontal_pos,
                grid_width + self.padding * 2.0,
                horizontal_pos,
                2.0,
                GRAY,
            );
            horizontal_pos += grid_height / self.rows
        }

        let piece_x = self.padding * 2.0 + grid_width / (self.columns * 2.0);
        let piece_y = self.padding * 2.0 + grid_height / (self.rows * 2.0);

        let font_size = 1.5 * grid_width.min(grid_height) / self.columns.max(self.rows);
        for (r, row) in self.grid.iter().enumerate() {
            for (c, piece) in row.iter().enumerate() {
                let xpos = piece_x + c as f32 * grid_width / self.columns;
                let ypos = piece_y + r as f32 * grid_height / self.rows;
                let Some((s, dims)) = self.chars.get(piece) else {
                    panic!("Unknown character '{piece}' in hashmap");
                };
                let dwidth = dims.width * (font_size / 10.0);
                let dheight = dims.height * (font_size / 10.0);
                draw_text(
                    s,
                    xpos - dwidth / 2.0,
                    ypos + dheight / 2.0,
                    font_size,
                    WHITE,
                );
            }
        }
    }

    pub fn update(&mut self, contents: &[Vec<char>]) {
        for c in contents {
            for piece in c {
                if let Entry::Vacant(e) = self.chars.entry(*piece) {
                    let s = format!("{piece}");
                    e.insert((s.clone(), measure_text(&s, None, 10, 1.0)));
                }
            }
        }
        self.grid = contents.into();
    }

    pub fn position_to_square(&self, loc: &(f32, f32)) -> Option<(usize, usize)> {
        if loc.0 < self.padding * 2.0 || loc.1 < self.padding * 2.0 {
            return None;
        }

        let w = screen_width();
        let h = screen_height();

        let grid_width = w - self.padding * 4.0;
        let grid_height = h - self.padding * 4.0;

        if loc.0 > self.padding * 2.0 + grid_width || loc.1 > self.padding * 2.0 + grid_height {
            return None;
        }

        let x = ((loc.0 - self.padding * 2.0) / (grid_width / self.columns)) as usize;
        let y = ((loc.1 - self.padding * 2.0) / (grid_height / self.rows)) as usize;

        Some((x + 1, y + 1))
    }
}
