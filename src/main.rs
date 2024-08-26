mod graph;
use graph::Graph;

fn main() {
    let graph = Graph::new().add_edges(&vec![(1, 2), (2, 3), (3, 4), (4, 1)]).build_adjacency_matrix();

    println!("{:?}", graph);
}
