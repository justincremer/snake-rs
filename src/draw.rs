use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, coords: (i32, i32), context: &Context, g_buf: &mut G2d) {
    let gui_x = to_coord(coords.0);
    let gui_y = to_coord(coords.1);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        context.transform,
        g_buf,
    );
}

pub fn draw_rectangle(
    color: Color,
    coords: (i32, i32),
    dimensions: (i32, i32),
    context: &Context,
    g_buf: &mut G2d,
) {
    let x = to_coord(coords.0);
    let y = to_coord(coords.1);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (dimensions.0 as f64),
            BLOCK_SIZE * (dimensions.1 as f64),
        ],
        context.transform,
        g_buf,
    );
}
