use std::usize;

use crate::grapho;
use grapho::GraphMatrix;

const WHITE: usize = 0;
const GRAY: usize = 1;
const BLACK: usize = 2;

#[derive(Debug)]
pub(crate) struct DFS {
    pi: Vec<i64>,
    color: Vec<usize>,
    d: Vec<usize>,
    f: Vec<usize>,
    time: usize,
    graph: GraphMatrix,
}

impl DFS {
    pub fn new(graph: GraphMatrix) -> Self {
        let n = graph.adjacency_matrix.len();
        DFS {
            pi: vec![-1; n],
            color: vec![WHITE; n],
            d: vec![0; n],
            f: vec![0; n],
            time: 0,
            graph: graph.clone(),
        }
    }

    pub fn dfs(&mut self) -> &mut Self {
        for u in 0..self.graph.adjacency_matrix.len() {
            if self.color[u] == WHITE {
                self.dfs_visit(u);
            }
        }
        self
    }

    fn dfs_visit(&mut self, u: usize) {
        self.color[u] = GRAY;
        self.time += 1;
        self.d[u] = self.time;

        let neighbors = self.graph.adjacency_matrix[u].len();
        for v in 0..neighbors {
            if self.color[v] == WHITE && self.graph.adjacency_matrix[u][v] == 1 {
                self.pi[v] = u as i64;
                self.dfs_visit(v);
            }
        }
        self.color[u] = BLACK;
        self.time += 1;
        self.f[u] = self.time;
    }
}
