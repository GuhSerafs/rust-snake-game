use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const TAM_BLOCO: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * TAM_BLOCO
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn desenhar_bloco(cor: Color, x: i32, y:i32, ctxt: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        cor, 
        [gui_x, gui_y, TAM_BLOCO, TAM_BLOCO], 
        ctxt.transform, 
        g
    );
}

pub fn desenhar_retangulo(
        cor: Color, 
        x: i32, 
        y:i32, 
        largura:i32, 
        altura:i32, 
        ctxt: &Context, 
        g: &mut G2d
    ) {
    let rect_x = to_coord(x);
    let rect_y = to_coord(y);

    rectangle(
        cor, 
        [rect_x, rect_y, TAM_BLOCO * (largura as f64), TAM_BLOCO * (altura as f64)], 
        ctxt.transform, 
        g
    );
}