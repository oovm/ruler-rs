use std::collections::{BTreeMap};

pub struct EGraph {
    head_nodes: BTreeMap<usize, usize>,
    last_edge: usize,
    nodes: BTreeMap<usize, usize>,
}

pub enum Expr {
    Add {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Sub {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Atom {
        value: f32
    },
}

impl Expr {
    pub fn into_expression(self) {
        match self {
            Expr::Add { left, right } => {
                left.into_expression();
                right.into_expression();
            }
            Expr::Sub { left, right } => {
                left.into_expression();
                right.into_expression();
            }
            Expr::Atom { value } => {}
        }
    }
}

pub struct Expression<H> {
    head: EHead<H>,
    rest: Vec<ENode<Self>>,
}

pub struct EHead<H> {
    id: usize,
    data: H,
}

pub struct ENode<N> {
    id: usize,
    data: N,
}

pub struct ENeighbors {
    end_nodes: BTreeMap<u32, u32>,
    links: BTreeMap<u32, u32>,
}
