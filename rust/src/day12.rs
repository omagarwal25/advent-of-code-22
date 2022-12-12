use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
enum Point {
    Start,
    End,
    Path(u32),
}

use Point::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord(usize, usize);

pub fn part1(inp: String) {
    let grid = inp
        .lines()
        .map(|l| {
            l.chars()
                .map(|f| match f {
                    'S' => Start,
                    'E' => End,
                    x => Path(x as u32 - 96),
                })
                .collect_vec()
        })
        .collect::<Vec<Vec<_>>>();

    let mut distances = vec![vec![usize::MAX; grid[0].len()]; grid.len()];

    let start_coords = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, p)| match p {
                Start => Some(Coord(x, y)),
                _ => None,
            })
        })
        .next()
        .unwrap();

    distances[start_coords.1][start_coords.0] = 0;

    let mut unvisited_nodes = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| Coord(x, y)))
        .collect_vec();

    while unvisited_nodes.len() > 0 {
        // println!("---");
        // print out coords
        // print out the distances map

        let current_node = unvisited_nodes
            .clone()
            .iter()
            .min_by_key(|c| distances[c.1][c.0])
            .unwrap()
            .clone();

        // pop the current node from the unvisited list
        unvisited_nodes.retain(|c| *c != current_node);

        // check if we're at the end
        if grid[current_node.1][current_node.0] == End {
            println!("Found the end!");
            println!("Distance: {}", distances[current_node.1][current_node.0]);
            break;
        }

        // if the distance is max, we can't get there
        if distances[current_node.1][current_node.0] == usize::MAX {
            continue;
        }

        let mut neighbours = vec![
            Coord(current_node.0 + 1, current_node.1),
            Coord(current_node.0, current_node.1 + 1),
        ];

        if current_node.0 > 0 {
            neighbours.push(Coord(current_node.0 - 1, current_node.1));
        }

        if current_node.1 > 0 {
            neighbours.push(Coord(current_node.0, current_node.1 - 1));
        }

        for neighbour in neighbours {
            if neighbour.0 >= grid[0].len() || neighbour.1 >= grid.len() {
                continue;
            }

            let neighbour_value = grid[neighbour.1][neighbour.0].clone();

            let current_node_value = grid[current_node.1][current_node.0].clone();

            // check that the neighbour's height is not more than 1 higher than the current node

            let current_height = match current_node_value {
                Path(x) => x,
                Start => 1,
                End => 26,
            };

            let neighbour_height = match neighbour_value {
                Path(x) => x,
                Start => 1,
                End => 26,
            };

            if neighbour_height as isize - current_height as isize > 1 {
                // print out the current height but in letters, and the neighbour height

                continue;
            }

            let new_distance = distances[current_node.1][current_node.0] + 1;

            if new_distance < distances[neighbour.1][neighbour.0] {
                distances[neighbour.1][neighbour.0] = new_distance;
            }
        }
    }
}

pub fn part2(inp: String) {
    let grid = inp
        .lines()
        .map(|l| {
            l.chars()
                .map(|f| match f {
                    'S' => Start,
                    'E' => End,
                    x => Path(x as u32 - 96),
                })
                .collect_vec()
        })
        .collect::<Vec<Vec<_>>>();

    let mut distances = vec![vec![usize::MAX; grid[0].len()]; grid.len()];

    let start_coords = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, p)| match p {
                End => Some(Coord(x, y)),
                _ => None,
            })
        })
        .next()
        .unwrap();

    distances[start_coords.1][start_coords.0] = 0;

    let mut unvisited_nodes = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| Coord(x, y)))
        .collect_vec();

    while unvisited_nodes.len() > 0 {
        let current_node = unvisited_nodes
            .clone()
            .iter()
            .min_by_key(|c| distances[c.1][c.0])
            .unwrap()
            .clone();

        // pop the current node from the unvisited list
        unvisited_nodes.retain(|c| *c != current_node);

        // check if we're at the end
        if grid[current_node.1][current_node.0] == Start
            || grid[current_node.1][current_node.0] == Path(1)
        {
            println!("Found the end!");
            println!("Distance: {}", distances[current_node.1][current_node.0]);
            break;
        }

        // if the distance is max, we can't get there
        if distances[current_node.1][current_node.0] == usize::MAX {
            continue;
        }

        let mut neighbours = vec![
            Coord(current_node.0 + 1, current_node.1),
            Coord(current_node.0, current_node.1 + 1),
        ];

        if current_node.0 > 0 {
            neighbours.push(Coord(current_node.0 - 1, current_node.1));
        }

        if current_node.1 > 0 {
            neighbours.push(Coord(current_node.0, current_node.1 - 1));
        }

        for neighbour in neighbours {
            if neighbour.0 >= grid[0].len() || neighbour.1 >= grid.len() {
                continue;
            }

            let neighbour_value = grid[neighbour.1][neighbour.0].clone();

            let current_node_value = grid[current_node.1][current_node.0].clone();

            // check that the neighbour's height is not more than 1 higher than the current node

            let current_height = match current_node_value {
                Path(x) => x,
                Start => 1,
                End => 26,
            };

            let neighbour_height = match neighbour_value {
                Path(x) => x,
                Start => 1,
                End => 26,
            };

            if current_height as isize - neighbour_height as isize > 1 {
                // print out the current height but in letters, and the neighbour height

                continue;
            }

            let new_distance = distances[current_node.1][current_node.0] + 1;

            if new_distance < distances[neighbour.1][neighbour.0] {
                distances[neighbour.1][neighbour.0] = new_distance;
            }
        }
    }
}
