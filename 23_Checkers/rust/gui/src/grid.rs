//! This is a modified copy of the code that is in the 26_Chomp directory. The intention is to someday
//! put this in the 00_Common/rust tree so it can be used for many programs without duplication

use macroquad::prelude::*;

use checkers::*;

pub struct Grid {
    padding: f32, // padding around the outside of the grid
    columns: f32,
    rows: f32,
    grid: Vec<Vec<Piece>>,
}

impl Grid {
    const MEDMAROON: Color = Color::new(0.5, 0.1, 0.15, 1.0);
    const DARKMAROON: Color = Color::new(0.36, 0.07, 0.11, 1.0);

    // Using "Metalic Silver" from https://kr.pinterest.com/pin/804103708514360361/
    const SILVER: Color = Color::new(
        (0xbc as f32) / 255.0,
        (0xc6 as f32) / 255.0,
        (0xcc as f32) / 255.0,
        1.0,
    );

    pub fn new(padding: f32, columns: usize, rows: usize) -> Self {
        Self {
            padding,
            columns: columns as f32,
            rows: rows as f32,
            grid: Vec::new(),
        }
    }

    pub fn _set_padding(&mut self, padding: f32) {
        self.padding = padding;
    }

    pub fn draw(&self) {
        let w = screen_width();
        let h = screen_height();

        // Calculate full width and height minus the surrounding padding
        // We put 2x padding on each side
        let grid_width = w - self.padding * 4.0;
        let grid_height = h - self.padding * 4.0;

        #[cfg(test)]
        {
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
        }

        let checker_x = self.padding * 2.0 + grid_width / (self.columns * 2.0);
        let checker_y = self.padding * 2.0 + grid_height / (self.rows * 2.0);
        let radius = 0.8 * grid_height.min(grid_width) / (self.columns.min(self.rows) * 2.0);

        let half_grid = (
            (grid_width / self.columns) / 2.0,
            (grid_height / self.rows) / 2.0,
        );

        for (r, row) in self.grid.iter().enumerate() {
            for (c, piece) in row.iter().enumerate() {
                let xpos = checker_x + c as f32 * grid_width / self.columns;
                let ypos = checker_y + r as f32 * grid_height / self.rows;
                let color = if !(r + c).is_multiple_of(2) {
                    Self::DARKMAROON
                } else {
                    Self::SILVER
                };
                draw_rectangle(
                    xpos - half_grid.0,
                    ypos - half_grid.1,
                    half_grid.0 * 2.0,
                    half_grid.1 * 2.0,
                    color,
                );

                let colors = match piece {
                    Piece::Empty => None,
                    Piece::Black | Piece::BlackKing => Some((BLACK, GRAY)),
                    Piece::Red | Piece::RedKing => Some((RED, Self::MEDMAROON)),
                };

                if let Some((color, highlight)) = colors {
                    draw_circle(xpos, ypos, radius, color);
                    draw_circle_lines(xpos, ypos, radius * 0.6, 2.0, highlight);
                    draw_circle_lines(xpos, ypos, radius * 0.95, 2.0, highlight);
                    if piece.is_king() {
                        draw_circle(xpos, ypos, radius / 2.0, highlight);
                        draw_line(
                            xpos - radius / 4.0,
                            ypos - radius / 4.0,
                            xpos - radius / 8.0,
                            ypos + radius / 4.0,
                            2.0,
                            color,
                        );
                        draw_line(
                            xpos - radius / 8.0,
                            ypos + radius / 4.0,
                            xpos + radius / 8.0,
                            ypos + radius / 4.0,
                            2.0,
                            color,
                        );

                        draw_line(
                            xpos + radius / 8.0,
                            ypos + radius / 4.0,
                            xpos + radius / 4.0,
                            ypos - radius / 4.0,
                            2.0,
                            color,
                        );
                        draw_line(
                            xpos,
                            ypos - radius / 4.0,
                            xpos + radius / 8.0,
                            ypos - radius / 8.0,
                            2.0,
                            color,
                        );
                        draw_line(
                            xpos + radius / 8.0,
                            ypos - radius / 8.0,
                            xpos + radius / 4.0,
                            ypos - radius / 4.0,
                            2.0,
                            color,
                        );

                        draw_line(
                            xpos,
                            ypos - radius / 4.0,
                            xpos - radius / 8.0,
                            ypos - radius / 8.0,
                            2.0,
                            color,
                        );
                        draw_line(
                            xpos - radius / 8.0,
                            ypos - radius / 8.0,
                            xpos - radius / 4.0,
                            ypos - radius / 4.0,
                            2.0,
                            color,
                        );
                    }
                }
            }
        }
    }

    pub fn update(&mut self, contents: &[Vec<Piece>]) {
        self.grid = contents.into();
        // println!("{:?}", self.grid);
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
