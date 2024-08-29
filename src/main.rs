mod graph;
use graph::{GraphList, cartesian_graph_list};

fn main() {
    let graph1 = GraphList::new()
        .add_edges(&vec![(1, 2), (2, 3), (3, 4), (4, 1)])
        .build_no_oriented();
    let graph2 = GraphList::new()
        .add_edges(&vec![(1, 4), (2, 6), (3, 2), (3, 1)])
        .build_no_oriented();

    print!("Graph 1: {:?}\n", graph1);
    print!("Graph 2: {:?}\n", graph2);

    let result = cartesian_graph_list(graph1, graph2);
    print!("Result graph: {:?}\n", result);
}
