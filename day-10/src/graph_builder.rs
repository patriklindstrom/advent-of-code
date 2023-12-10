
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use std::fs::File;
use std::io::{self, BufRead};
fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect::<Result<_, _>>()
}

fn initialize_nodes(lines: &[String]) -> (Graph<(usize, usize), ()>, Vec<Vec<NodeIndex>>) {
    let mut graph = Graph::new();
    let mut nodes = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        let row_nodes: Vec<_> = line.chars().enumerate().map(|(x, _)| {
            graph.add_node((x, y))
        }).collect();
        nodes.push(row_nodes);
    }

    (graph, nodes)
}

fn add_vertical_edges(graph: &mut Graph<(usize, usize), ()>, nodes: &Vec<Vec<NodeIndex>>, lines: &[String]) {
    for (y, row) in nodes.iter().enumerate() {
        for (x, &node) in row.iter().enumerate() {
            if lines[y].chars().nth(x).unwrap() == '|' {
                if y > 0 && "|LJ7F".contains(lines[y - 1].chars().nth(x).unwrap()) {
                    graph.add_edge(node, nodes[y - 1][x], ());
                }
                if y < lines.len() - 1 && "|LJ7F".contains(lines[y + 1].chars().nth(x).unwrap()) {
                    graph.add_edge(node, nodes[y + 1][x], ());
                }
            }
        }
    }
}

fn add_horizontal_edges(graph: &mut Graph<(usize, usize), ()>, nodes: &Vec<Vec<NodeIndex>>, lines: &[String]) {
    for (y, row) in nodes.iter().enumerate() {
        for (x, &node) in row.iter().enumerate() {
            if lines[y].chars().nth(x).unwrap() == '-' {
                if x > 0 && "-LJ7F".contains(lines[y].chars().nth(x - 1).unwrap()) {
                    graph.add_edge(node, nodes[y][x - 1], ());
                }
                if x < row.len() - 1 && "-LJ7F".contains(lines[y].chars().nth(x + 1).unwrap()) {
                    graph.add_edge(node, nodes[y][x + 1], ());
                }
            }
        }
    }
}

fn add_bend_edges(graph: &mut Graph<(usize, usize), ()>, nodes: &Vec<Vec<NodeIndex>>, lines: &[String], bend_char: char) {
    for (y, row) in nodes.iter().enumerate() {
        for (x, &node) in row.iter().enumerate() {
            if lines[y].chars().nth(x).unwrap() == bend_char {
                match bend_char {
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
                    _ => {}
                }
            }
        }
    }
}


fn add_start_position_edges(graph: &mut Graph<(usize, usize), ()>, nodes: &Vec<Vec<NodeIndex>>, lines: &[String]) {
    for (y, row) in nodes.iter().enumerate() {
        for (x, &node) in row.iter().enumerate() {
            if lines[y].chars().nth(x).unwrap() == 'S' {
                // Implement connections for the starting position 'S'
            }
        }
    }
}

pub fn build_graph(lines: &[String]) -> io::Result<Graph<(usize, usize), ()>> {
    let (mut graph, nodes) = initialize_nodes(lines);

    add_vertical_edges(&mut graph, &nodes, lines);
    add_horizontal_edges(&mut graph, &nodes, lines);
    add_bend_edges(&mut graph, &nodes, lines, 'L');
    add_bend_edges(&mut graph, &nodes, lines, 'J');
    add_bend_edges(&mut graph, &nodes, lines, '7');
    add_bend_edges(&mut graph, &nodes, lines, 'F');
    add_start_position_edges(&mut graph, &nodes, lines);

    Ok(graph)
}