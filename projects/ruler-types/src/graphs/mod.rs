use std::collections::BTreeMap;
use adjacency_list::AdjacencyNodeList;

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
