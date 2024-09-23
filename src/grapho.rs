use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct GraphMatrix {
    pub adjacency_matrix: Vec<Vec<usize>>,
    vertices: HashSet<usize>,
    pub edges: HashSet<(usize, usize)>,
}
#[derive(Debug)]
pub struct GraphList {
    pub adjacency_list: HashMap<usize, HashSet<usize>>,
    vertices: HashSet<usize>,
    pub edges: HashSet<(usize, usize)>,
}

impl GraphMatrix {
    pub fn new() -> Self {
        GraphMatrix {
            adjacency_matrix: Vec::new(),
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    fn add_vertex(&mut self, vertex: usize) {
        self.vertices.insert(vertex);
    }

    fn set_edge(&mut self, vertex1: usize, vertex2: usize) {
        if self.vertices.contains(&vertex1) == false {
            self.add_vertex(vertex1);
        }

        if self.vertices.contains(&vertex2) == false {
            self.add_vertex(vertex2);
        }
    }

    pub fn add_edges(mut self, edges: &Vec<(usize, usize)>) -> Self {
        for edge in edges {
            self.set_edge(edge.0, edge.1);
            self.edges.insert(*edge);
        }

        self
    }

    pub fn build_oriented(mut self) -> Self {
        let max_vertex = *self.vertices.iter().max().unwrap() as usize + 1;
        self.adjacency_matrix = vec![vec![0; max_vertex]; max_vertex];

        for edge in self.edges.iter() {
            self.adjacency_matrix[edge.0][edge.1] = 1;
        }

        self
    }

    pub fn build_no_oriented(mut self) -> Self {
        let max_vertex = *self.vertices.iter().max().unwrap() as usize + 1;
        self.adjacency_matrix = vec![vec![0; max_vertex]; max_vertex];

        for edge in self.edges.iter() {
            self.adjacency_matrix[edge.0][edge.1] = 1;
            self.adjacency_matrix[edge.1][edge.0] = 1;
        }

        self
    }

    pub fn contains_edge(&self, vertex: &(usize, usize)) -> bool {
        let (vertex1, vertex2) = vertex;
        self.adjacency_matrix[*vertex1][*vertex2] > 0
    }
}

impl GraphList {
    pub fn new() -> Self {
        GraphList {
            adjacency_list: HashMap::new(),
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    fn add_vertex(&mut self, vertex: usize) {
        self.vertices.insert(vertex);
        self.adjacency_list.insert(vertex, HashSet::new());
    }

    fn set_edge(&mut self, vertex1: usize, vertex2: usize) {
        if self.vertices.contains(&vertex1) == false {
            self.add_vertex(vertex1);
        }

        if self.vertices.contains(&vertex2) == false {
            self.add_vertex(vertex2);
        }
    }

    pub fn add_edges(mut self, edges: &Vec<(usize, usize)>) -> Self {
        for edge in edges {
            self.set_edge(edge.0, edge.1);
            self.edges.insert(*edge);
        }

        self
    }

    pub fn build_oriented(mut self) -> Self {
        for edge in self.edges.iter() {
            self.adjacency_list.get_mut(&edge.0).unwrap().insert(edge.1);
        }
        self
    }

    pub fn build_no_oriented(mut self) -> Self {
        for edge in self.edges.iter() {
            self.adjacency_list.get_mut(&edge.0).unwrap().insert(edge.1);
            self.adjacency_list.get_mut(&edge.1).unwrap().insert(edge.0);
        }
        self
    }

    pub fn contains_edge(&self, vertex: &(usize, usize)) -> bool {
        let (vertex1, vertex2) = vertex;
        match self.adjacency_list.get(&vertex1) {
            Some(edges) => edges.contains(&vertex2),
            None => false,
        }
    }
}

pub fn cartesian_graph_list(g1: GraphList, g2: GraphList) -> GraphList {
    let mut new_vertices: Vec<(usize, usize)> = Vec::new();

    for v in g1.vertices.iter() {
        for u in g2.vertices.iter() {
            new_vertices.push((*v, *u));
        }
    }
    let mut new_edges: Vec<(usize, usize)> = Vec::new();
    for (index_1, elem_1) in new_vertices.iter().enumerate() {
        for (index_2, elem_2) in new_vertices.iter().enumerate() {
            if elem_1.0 == elem_2.0 && g2.contains_edge(&(elem_1.1, elem_2.1)) {
                new_edges.push((index_1, index_2));
            }
            if elem_1.1 == elem_2.1 && g1.contains_edge(&(elem_1.0, elem_2.0)) {
                new_edges.push((index_1, index_2));
            }
        }
    }

    GraphList::new().add_edges(&new_edges).build_no_oriented()
}
