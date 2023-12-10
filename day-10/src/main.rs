use petgraph::Graph;
use std::fs::File;
use std::io::{self, BufRead};

fn build_graph(file_path: &str) -> io::Result<Graph<(usize, usize), ()>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut graph = Graph::new();
    let mut nodes = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let row_nodes: Vec<_> = chars.iter().enumerate().map(|(x, _)| {
            graph.add_node((x, y))
        }).collect();
        nodes.push(row_nodes);
    }

    // Add edges based on the pipe layout
    for (y, row) in nodes.iter().enumerate() {
        for (x, &node) in row.iter().enumerate() {
            let c = lines[y].chars().nth(x).unwrap();
            match c {
                '|' => {
                    // Logic for vertical pipe '|'
                    // [snipped for brevity]
                },
                '-' => {
                    // Logic for horizontal pipe '-'
                    // [snipped for brevity]
                },
                'L' => {
                    if y > 0 && "|7F".contains(lines[y - 1].chars().nth(x).unwrap()) {
                        graph.add_edge(node, nodes[y - 1][x], ());
                    }
                    if x < row.len() - 1 && "-JL".contains(lines[y].chars().nth(x + 1).unwrap()) {
                        graph.add_edge(node, nodes[y][x + 1], ());
                    }
                },
                'J' => {
                    if y > 0 && "|7F".contains(lines[y - 1].chars().nth(x).unwrap()) {
                        graph.add_edge(node, nodes[y - 1][x], ());
                    }
                    if x > 0 && "-JL".contains(lines[y].chars().nth(x - 1).unwrap()) {
                        graph.add_edge(node, nodes[y][x - 1], ());
                    }
                },
                '7' => {
                    if y < lines.len() - 1 && "|LJ".contains(lines[y + 1].chars().nth(x).unwrap()) {
                        graph.add_edge(node, nodes[y + 1][x], ());
                    }
                    if x > 0 && "-F7".contains(lines[y].chars().nth(x - 1).unwrap()) {
                        graph.add_edge(node, nodes[y][x - 1], ());
                    }
                },
                'F' => {
                    if y < lines.len() - 1 && "|LJ".contains(lines[y + 1].chars().nth(x).unwrap()) {
                        graph.add_edge(node, nodes[y + 1][x], ());
                    }
                    if x < row.len() - 1 && "-F7".contains(lines[y].chars().nth(x + 1).unwrap()) {
                        graph.add_edge(node, nodes[y][x + 1], ());
                    }
                },
                'S' => {
                    // Check and connect to two adjacent pipes
                    let mut connected = 0;

                    // Check north
                    if y > 0 && "|7F".contains(lines[y - 1].chars().nth(x).unwrap()) {
                        graph.add_edge(node, nodes[y - 1][x], ());
                        connected += 1;
                    }

                    // Check south
                    if y < lines.len() - 1 && "|LJ".contains(lines[y + 1].chars().nth(x).unwrap()) {
                        graph.add_edge(node, nodes[y + 1][x], ());
                        connected += 1;
                    }

                    // Check west
                    if connected < 2 && x > 0 && "-JL".contains(lines[y].chars().nth(x - 1).unwrap()) {
                        graph.add_edge(node, nodes[y][x - 1], ());
                        connected += 1;
                    }

                    // Check east
                    if connected < 2 && x < row.len() - 1 && "-F7".contains(lines[y].chars().nth(x + 1).unwrap()) {
                        graph.add_edge(node, nodes[y][x + 1], ());
                    }
                },
                _ => {}
            }
        }
    }

    Ok(graph)
}

fn main() -> io::Result<()> {
    let graph = build_graph("testdata/pipemap1.txt")?;
    // Implement the logic to find the farthest point using graph traversal

    Ok(())
}
