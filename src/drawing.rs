extern crate minilibx;

use minilibx::{Mlx, MlxWindow};

#[derive(Debug)]
pub enum LineDrawingMode {
    Bresenham,
    XiaolinWu,
}

pub fn draw_line(
    mlx: &Mlx,
    window: &MlxWindow,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    mode: Option<LineDrawingMode>,
) {
    let mode = mode.unwrap_or(LineDrawingMode::Bresenham);

    match mode {
        LineDrawingMode::Bresenham => {
            let mut x = x0;
            let mut y = y0;
            let dx = (x1 - x0).abs();
            let dy = (y1 - y0).abs();
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };
            let mut err = dx - dy;

            loop {
                mlx.pixel_put(&window, x, y, 0xffffff);

                if x == x1 && y == y1 { break; }
                let e2 = 2 * err;
                if e2 > -dy {
                    err -= dy;
                    x += sx;
                }
                if e2 < dx {
                    err += dx;
                    y += sy;
                }
            }
        }
        LineDrawingMode::XiaolinWu => {
            // Placeholder for Xiaolin Wu's line drawing algorithm
            // The actual implementation would require anti-aliasing logic
            println!("Drawing line with Xiaolin Wu's algorithm (placeholder)");
        }
    }
}


/*
fn main() {
    let mlx = Mlx::new().unwrap();
    let window = mlx.new_window(1080, 720, "Line Drawing").unwrap();

    // Draw a line using Bresenham's algorithm (default)
    draw_line(&mlx, &window, 10, 10, 200, 200, None);

    // Draw a line using Xiaolin Wu's algorithm
    draw_line(&mlx, &window, 200, 200, 400, 400, Some(LineDrawingMode::XiaolinWu));

    mlx.event_loop();
}



*/