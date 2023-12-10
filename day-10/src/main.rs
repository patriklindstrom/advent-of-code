use petgraph::Graph;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Read the maze and build the graph
fn build_graph(file_path: &str) -> io::Result<Graph<(usize, usize), u32>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut graph = Graph::new();
    let mut nodes = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        let row_nodes: Vec<_> = chars.iter().enumerate().map(|(x, &c)| {

            graph.add_node((x, y))
        }).collect();
        nodes.push(row_nodes);
    }



    Ok(graph)
}

fn main() -> io::Result<()> {
    let graph = build_graph("testdata/pipemap1.txt")?;


    Ok(())
}
