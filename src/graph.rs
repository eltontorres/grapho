use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Graph {
    adjacency_matrix: Vec<Vec<i64>>,
    adjacency_list: HashMap<i64, HashSet<i64>>,
    vertices: HashSet<i64>,
    edges: HashSet<(i64, i64)>,
    directed: bool,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_matrix: Vec::new(),
            adjacency_list: HashMap::new(),
            vertices: HashSet::new(),
            edges: HashSet::new(),
            directed: true,
        }
    }

    fn add_vertex(&mut self, vertex: i64) {
        self.vertices.insert(vertex);
        self.adjacency_list.insert(vertex, HashSet::new());
    }

    fn set_edge(&mut self, vertex1: i64, vertex2: i64) {
        if self.vertices.contains(&vertex1) == false {
            self.add_vertex(vertex1);
        }

        if self.vertices.contains(&vertex2) == false {
            self.add_vertex(vertex2);
        }
    }

    pub fn add_edges(mut self, edges: &Vec<(i64, i64)>) -> Self {
        for edge in edges {
            self.set_edge(edge.0, edge.1);
            self.edges.insert(*edge);
        }

        self
    }

    pub fn build_adjacency_list(mut self) -> Self {
        for edge in self.edges.iter() {
            self.adjacency_list.get_mut(&edge.0).unwrap().insert(edge.1);
            if self.directed == false {
                self.adjacency_list.get_mut(&edge.1).unwrap().insert(edge.0);
            }
        }
        self
    }

    pub fn build_adjacency_matrix(mut self) -> Self {
        let max_vertex = *self.vertices.iter().max().unwrap() as usize + 1;
        self.adjacency_matrix = vec![vec![0; max_vertex]; max_vertex];

        for edge in self.edges.iter() {
            self.adjacency_matrix[edge.0 as usize][edge.1 as usize] = 1;
            if self.directed == false {
                self.adjacency_matrix[edge.1 as usize][edge.0 as usize] = 1;
            }
        }

        self
    }
}
