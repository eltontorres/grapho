use std::{clone, i64};

use crate::grapho;
use grapho::GraphMatrix;
use std::collections::VecDeque;

const WHITE: usize = 0;
const GRAY: usize = 1;
const BLACK: usize = 2;

#[derive(Debug)]
pub(crate) struct BFS {
    pi: Vec<i64>,
    color: Vec<usize>,
    d: Vec<usize>,
    graph: GraphMatrix,
    deque: VecDeque<usize>,
}

impl BFS {
    pub fn new(graph: GraphMatrix) -> Self {
        let n = graph.adjacency_matrix.len();
        BFS {
            pi: vec![i64::MAX; n],
            color: vec![WHITE; n],
            d: vec![0; n],
            graph,
            deque: VecDeque::new(),
        }
    }

    pub fn bfs(&mut self, s: usize) -> &mut Self {
        self.color[s] = GRAY;
        self.d[s] = 0;
        self.pi[s] = -1;
        self.deque.push_back(s);

        while !self.deque.is_empty() {
            let u = self.deque.pop_front().unwrap();
            let neighbors = self.graph.adjacency_matrix[u].len();
            for v in 0..neighbors {
                if self.color[v] == WHITE && self.graph.adjacency_matrix[u][v] == 1 {
                    self.color[v] = GRAY;
                    self.d[v] = self.d[u] + 1;
                    self.pi[v] = u as i64;
                    self.deque.push_back(v);
                }
            }
            self.color[u] = BLACK;
        }
        self
    }
}
