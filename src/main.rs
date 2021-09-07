extern crate rand;
extern crate piston_window;

mod desenhar;
mod cobrinha;
mod jogo;

use piston_window::*;
use piston_window::types::Color;

use jogo::Jogo;
use desenhar::to_coord_u32;

const BG_COLOR: Color = [0.50, 0.50, 0.50, 1.0];

fn main() {
    let (largura, altura) = (20, 20);
    let mut window: PistonWindow = WindowSettings::new(
        "Cobrinha", 
        [to_coord_u32(largura), to_coord_u32(altura)],
    ).vsync(true).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Jogo::new(largura, altura);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.btn_pressionado(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BG_COLOR, g); 
            game.desenhar_tela(&c, g);
        });

        event.update(|arg| {
            game.atualizar(arg.dt);
        });
    }
}

