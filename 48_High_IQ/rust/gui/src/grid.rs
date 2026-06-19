//! This is a modified copy of the code that is in the 26_Chomp directory. The intention is to someday
//! put this in the 00_Common/rust tree so it can be used for many programs without duplication

use macroquad::prelude::*;

pub struct Grid {
    padding: f32, // padding around the outside of the grid
    columns: f32,
    rows: f32,
    grid: [[isize; 10]; 10],
}

impl Grid {
    pub fn new(padding: f32, columns: usize, rows: usize) -> Self {
        Self {
            padding,
            columns: columns as f32,
            rows: rows as f32,
            grid: [[0; 10]; 10],
        }
    }

    pub fn _set_padding(&mut self, padding: f32) {
        self.padding = padding;
    }

    pub fn draw(&self, hide: Option<(usize, usize)>) {
        let w = screen_width();
        let h = screen_height();

        // Calculate full width and height minus the surrounding padding
        // We put 2x padding on each side
        let grid_width = w - self.padding * 4.0;
        let grid_height = h - self.padding * 4.0;

        let rect_width = grid_width / self.columns;
        let rect_height = grid_height / self.rows;

        let cols = self.columns as i32;
        for vert_line in 0..cols + 1 {
            let vertical_pos = self.padding * 2.0 + vert_line as f32 * rect_width;
            let (start_y, end_y) = if vert_line < 2 || vert_line > cols - 2 {
                (
                    self.padding * 2.0 + rect_height * 2.0,
                    grid_height + self.padding * 2.0 - rect_height * 2.0,
                )
            } else {
                (self.padding * 2.0, grid_height + self.padding * 2.0)
            };

            draw_line(vertical_pos, start_y, vertical_pos, end_y, 2.0, GRAY);
        }

        let rows = self.rows as i32;
        for horiz_line in 0..rows + 1 {
            let horizontal_pos = self.padding * 2.0 + horiz_line as f32 * rect_height;
            let (start_x, end_x) = if horiz_line < 2 || horiz_line > rows - 2 {
                (
                    self.padding * 2.0 + rect_width * 2.0,
                    grid_width + self.padding * 2.0 - rect_width * 2.0,
                )
            } else {
                (self.padding * 2.0, grid_width + self.padding * 2.0)
            };
            draw_line(start_x, horizontal_pos, end_x, horizontal_pos, 2.0, GRAY);
        }

        let checker_x = self.padding * 2.0 + grid_width / (self.columns * 2.0);
        let checker_y = self.padding * 2.0 + grid_height / (self.rows * 2.0);
        let radius = 0.8 * grid_height.min(grid_width) / (self.columns.min(self.rows) * 2.0);

        for (r, row) in self.grid.iter().skip(2).take(7).enumerate() {
            for (c, piece) in row.iter().skip(2).take(7).enumerate() {
                let xpos = checker_x + c as f32 * grid_width / self.columns;
                let ypos = checker_y + r as f32 * grid_height / self.rows;

                if Some((r, c)) == hide {
                    continue;
                }

                let colors = match piece {
                    -5 | 0 => None,
                    5 => Some(BLUE),
                    _ => panic!("invalid piece"),
                };

                if let Some(color) = colors {
                    draw_circle(xpos, ypos, radius, color);
                }
            }
        }
    }

    pub fn update(&mut self, contents: &[[isize; 10]; 10]) {
        self.grid = *contents;
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
