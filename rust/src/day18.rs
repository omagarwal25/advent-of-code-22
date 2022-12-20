use itertools::Itertools;

pub fn part1(inp: String) {
    let coords = inp
        .lines()
        .map(|f| {
            f.split(",")
                .map(|a| a.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32, i32)>()
                .unwrap()
        })
        .collect_vec();

    let mut grid = vec![vec![vec![false; 50]; 50]; 50];

    for (x, y, z) in coords {
        grid[x as usize][y as usize][z as usize] = true;
    }

    let mut surface_area = 0;

    for z in 0..50 {
        for y in 0..50 {
            for x in 0..50 {
                if grid[x][y][z] {
                    // check if there is a neighbour

                    let mut neighbours = 0;

                    if x > 0 && grid[x - 1][y][z] {
                        neighbours += 1;
                    }

                    if x < 49 && grid[x + 1][y][z] {
                        neighbours += 1;
                    }

                    if y > 0 && grid[x][y - 1][z] {
                        neighbours += 1;
                    }

                    if y < 49 && grid[x][y + 1][z] {
                        neighbours += 1;
                    }

                    if z > 0 && grid[x][y][z - 1] {
                        neighbours += 1;
                    }

                    if z < 49 && grid[x][y][z + 1] {
                        neighbours += 1;
                    }

                    surface_area += 6 - neighbours;
                }
            }
        }
    }

    println!("{}", surface_area);
}

pub fn part2(inp: String) {
    let coords = inp
        .lines()
        .map(|f| {
            f.split(",")
                .map(|a| a.parse::<usize>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect_vec();

    let mut grid = vec![vec![vec![false; 50]; 50]; 50];

    for (x, y, z) in coords.clone() {
        grid[x as usize][y as usize][z as usize] = true;
    }

    let mut queue: Vec<(usize, usize, usize)> = vec![(0, 0, 0)];

    let mut visited = vec![vec![vec![false; 50]; 50]; 50];

    let mut count = 0;

    while let Some(b) = queue.pop() {
        if visited[b.0][b.1][b.2] {
            continue;
        }

        visited[b.0][b.1][b.2] = true;

        // check if there is a neighbour

        let mut neighbours = vec![];

        if b.0 > 0 {
            neighbours.push((b.0 - 1, b.1, b.2));
        }

        if b.0 < 49 {
            neighbours.push((b.0 + 1, b.1, b.2));
        }

        if b.1 > 0 {
            neighbours.push((b.0, b.1 - 1, b.2));
        }

        if b.1 < 49 {
            neighbours.push((b.0, b.1 + 1, b.2));
        }

        if b.2 > 0 {
            neighbours.push((b.0, b.1, b.2 - 1));
        }

        if b.2 < 49 {
            neighbours.push((b.0, b.1, b.2 + 1));
        }

        for n in neighbours {
            if grid[n.0][n.1][n.2] {
                count += 1;
            } else {
                queue.push(n);
            }
        }
    }

    // just check if any of coords are touching the edge

    for (x, y, z) in coords {
        if x == 0 || x == 49 || y == 0 || y == 49 || z == 0 || z == 49 {
            count += 1;
        }
    }

    println!("{}", count);
}
