use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::cobrinha::{Cobrinha, Direção};
use crate::desenhar::{desenhar_bloco, desenhar_retangulo};

const COR_COMIDA: Color = [0.80, 0.00, 0.00, 1.0];
const COR_BORDA: Color = [0.00, 0.00, 0.00, 1.0];
const COR_GAMEOVER: Color = [0.90, 0.00, 0.00, 0.5];

const INTERVALO_MOVT: f64 = 0.1;
const TEMPO_RESTART: f64 = 1.0;

pub struct Jogo {
    cobrinha: Cobrinha, 
    comida_existe: bool, 
    comida_x: i32, 
    comida_y: i32, 

    largura: i32, 
    altura: i32,
    
    game_over: bool,
    tempo_espera: f64,
}

impl Jogo {
    pub fn new(largura: i32, altura: i32) -> Jogo {
        Jogo {
            cobrinha: Cobrinha::new(2, 2),
            tempo_espera: 0.0, 
            comida_existe: true, 
            comida_x: 6,
            comida_y: 4,
            largura, 
            altura,
            game_over: false
        }
    }

    pub fn btn_pressionado(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direção::Cima), 
            Key::Down => Some(Direção::Baixo), 
            Key::Left => Some(Direção::Esquerda), 
            Key::Right => Some(Direção::Direita), 
            _ => None
        };

        if dir.unwrap() == self.cobrinha.direção_cabeça().oposta() {
            return
        } else {
            self.atualizar_cobrinha(dir);
        }
    }

    pub fn desenhar_tela(&self, ctxt: &Context, g: &mut G2d) {
        self.cobrinha.desenhar(ctxt, g); 

        if self.comida_existe {
            desenhar_bloco(COR_COMIDA, self.comida_x, self.comida_y, ctxt, g);
        }

        desenhar_retangulo(COR_BORDA, 0, 0, self.largura, 1, ctxt, g);
        desenhar_retangulo(COR_BORDA, 0, self.altura - 1, self.largura, 1, ctxt, g);
        desenhar_retangulo(COR_BORDA, 0, 0, 1, self.altura, ctxt, g);
        desenhar_retangulo(COR_BORDA, self.largura - 1, 0, 1, self.altura, ctxt, g);

        if self.game_over {
            desenhar_retangulo(COR_GAMEOVER, 0, 0, self.largura, self.altura, ctxt, g);
        }
    }

    pub fn atualizar(&mut self, delta_time: f64) {
        self.tempo_espera += delta_time;

        if self.game_over {
            if self.tempo_espera > TEMPO_RESTART {
                self.reiniciar();
            }
            return;
        }

        if !self.comida_existe {
            self.colocar_comida();
        }

        if self.tempo_espera > INTERVALO_MOVT {
            self.atualizar_cobrinha(None);
        }
    }

    pub fn verificar_se_comeu(&mut self) {
        let (cabeça_x, cabeça_y) = self.cobrinha.posicao_cabeça();
        if self.comida_existe && self.comida_x == cabeça_x && self.comida_y == cabeça_y {
            self.comida_existe = false;
            self.cobrinha.restaurar_cauda();
        }
    }

    pub fn verificar_se_esta_viva(&self, dir: Option<Direção>) -> bool {
        let (prox_x, prox_y) = self.cobrinha.prox_cabeça(dir);
        if self.cobrinha.sobrepor_cauda(prox_x, prox_y) {
            return false;
        }

        prox_x > 0 && prox_y > 0 && prox_x < (self.largura - 1) && prox_y < (self.altura - 1)
    }

    pub fn colocar_comida(&mut self) {
        let mut rng = thread_rng(); 

        let mut nova_comida_x = rng.gen_range(1..self.largura - 1);
        let mut nova_comida_y = rng.gen_range(1..self.altura - 1);
        
        while self.cobrinha.sobrepor_cauda(nova_comida_x, nova_comida_y) {
            nova_comida_x = rng.gen_range(1..self.largura - 1);
            nova_comida_y = rng.gen_range(1..self.altura - 1);
        }

        self.comida_x = nova_comida_x;
        self.comida_y = nova_comida_y;
        self.comida_existe = true;
    }

    pub fn atualizar_cobrinha(&mut self, dir: Option<Direção>) {
        if self.verificar_se_esta_viva(dir) {
            self.cobrinha.avançar(dir);
            self.verificar_se_comeu();
        } else {
            self.game_over = true;
        }
        self.tempo_espera = 0.0;
    }

    pub fn reiniciar(&mut self) {
        self.cobrinha = Cobrinha::new(2, 2);
        self.tempo_espera = 0.0;
        self.comida_existe = true;
        self.comida_x = 6;
        self.comida_y = 4;
        self.game_over = false;
    }
}