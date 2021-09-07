use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::desenhar::desenhar_bloco;

const COR_COBRINHA: Color = [0.00, 0.80, 0.00, 1.00];

#[derive(Copy, Clone, PartialEq)]
pub enum Direção {
    Cima,
    Baixo,
    Esquerda,
    Direita
}

impl Direção {
    pub fn oposta(&self) -> Direção {
        match *self {
            Direção::Cima => Direção::Baixo, 
            Direção::Baixo => Direção::Cima, 
            Direção::Esquerda => Direção::Direita,
            Direção::Direita => Direção::Esquerda
        }
    }
}

#[derive(Debug, Clone)]
struct Bloco {
    x: i32, 
    y: i32,
}

pub struct Cobrinha {
    direção: Direção, 
    corpo: LinkedList<Bloco>,
    cauda: Option<Bloco>,
}

impl Cobrinha {
    pub fn new(x:i32, y:i32) -> Cobrinha {
        // Construindo o corpo
        let mut corpo: LinkedList<Bloco> = LinkedList::new();

        // Colocando os três primeiros blocos
        corpo.push_back(Bloco {
            x: x+2, 
            y: y,
        });
        corpo.push_back(Bloco {
            x: x+1, 
            y: y,
        });
        corpo.push_back(Bloco {
            x: x, 
            y: y,
        });

        // Define uma direção e conclui com a cauda
        Cobrinha {
            direção: Direção::Direita, 
            corpo,
            cauda: None,
        }
    }

    pub fn desenhar(&self, ctxt: &Context, g: &mut G2d) {
        for bloco in &self.corpo {
            desenhar_bloco(COR_COBRINHA, bloco.x, bloco.y, ctxt, g);
        }        
    }

    pub fn posicao_cabeça(&self) -> (i32, i32) {
        let cabeça = self.corpo.front().unwrap();
        (cabeça.x, cabeça.y)
    }

    pub fn avançar(&mut self, dir: Option<Direção>) {
        match dir {
            Some(d) => self.direção = d,
            None => (),
        }

        let (x_old, y_old) = self.posicao_cabeça();

        let novo_bloco = match self.direção {
            Direção::Cima => Bloco {x: x_old, y: y_old - 1},
            Direção::Baixo => Bloco {x: x_old, y: y_old + 1},
            Direção::Esquerda => Bloco {x: x_old - 1, y: y_old},
            Direção::Direita => Bloco {x: x_old + 1, y: y_old},
        };

        self.corpo.push_front(novo_bloco);
        let bloco_removido = self.corpo.pop_back().unwrap();
        self.cauda = Some(bloco_removido);
    }

    pub fn direção_cabeça(&self) -> Direção {
        self.direção
    }

    pub fn prox_cabeça(&self, dir: Option<Direção>) -> (i32, i32) {
        let (cabeça_x, cabeça_y) = self.posicao_cabeça();
        let mut direção_movt = self.direção;
        match dir {
            Some(d) => direção_movt = d, 
            None => {}
        }

        match direção_movt {
            Direção::Cima => (cabeça_x, cabeça_y - 1), 
            Direção::Baixo => (cabeça_x, cabeça_y + 1), 
            Direção::Esquerda => (cabeça_x - 1, cabeça_y), 
            Direção::Direita => (cabeça_x + 1, cabeça_y)
        }
    }

    pub fn restaurar_cauda(&mut self) {
        let bloco_cauda = self.cauda.clone().unwrap();
        self.corpo.push_back(bloco_cauda);
    }

    pub fn sobrepor_cauda(&self, x: i32, y:i32) -> bool {
        let mut i = 0;
        for bloco in &self.corpo {
            if x == bloco.x && y == bloco.y {
                return true;
            } else {
                i += 1;
                if i >= (self.corpo.len() - 1) {
                    break;
                }
            }
        }
        return false;
    }





}