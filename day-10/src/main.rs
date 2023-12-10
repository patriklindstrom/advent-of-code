mod graph_builder;
use std::io;
use std::io::BufRead;

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    reader.lines().collect::<Result<_, _>>()
}

fn main() -> io::Result<()> {
    let lines = read_file("testdata/pipemap1.txt")?;
    let graph = graph_builder::build_graph(&lines)?;
    // Logic to find the farthest point using graph traversal

    Ok(())
}
