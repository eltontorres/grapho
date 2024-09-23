mod bfs;
mod dfs;
mod grapho;
use bfs::BFS;
use dfs::DFS;
use grapho::GraphMatrix;

fn main() {
    let graph1 = GraphMatrix::new()
        .add_edges(&vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)])
        .build_oriented();

    let mut binding = DFS::new(graph1.clone());
    let dfs = binding.dfs();
    print!("{:?}\n", dfs);

    let mut binding = BFS::new(graph1.clone());
    let bfs = binding.bfs(0);

    print!("{:?}\n", bfs);
}
