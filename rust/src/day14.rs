use itertools::Itertools;

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Sand,
    Air,
    Rock,
}

pub fn part1(inp: String) {
    let lines = inp
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|a| a.split(",").collect_tuple().unwrap())
                .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
                .collect_vec()
        })
        .collect_vec();

    let mut grid = vec![vec![Tile::Air; 1000]; 1000];

    // draw the lines

    for line in lines {
        // zip each point with the next point

        for (a, b) in line.iter().tuple_windows() {
            // draw a line between the two points

            // if the points are the same, draw a vertical line

            // println!("Drawing Line between {:?} and {:?}", a, b);

            if a.0 == b.0 {
                // draw a vertical line

                let smaller = a.1.min(b.1);
                let larger = a.1.max(b.1);

                for y in smaller..=larger {
                    grid[a.0 as usize][y as usize] = Tile::Rock;
                }
            } else {
                // draw a horizontal line

                let smaller = a.0.min(b.0);
                let larger = a.0.max(b.0);

                for x in smaller..=larger {
                    grid[x as usize][a.1 as usize] = Tile::Rock;
                }
            }
        }
    }

    let mut count = 0;

    loop {
        // spawn in a sand
        // follows the following rules
        // 1. if there is air below, fall
        // 2. if one air diagonally left fall
        // 3. if one air diagonally right fall
        // repeat until no more movement

        let mut current_pos = (500, 0);
        let mut unsettled = false;

        loop {
            // check if there is air below

            if current_pos.1 == 999 {
                unsettled = true;
                break;
            }

            if grid[current_pos.0][current_pos.1 + 1] == Tile::Air {
                // fall

                grid[current_pos.0][current_pos.1] = Tile::Air;
                current_pos.1 += 1;
                grid[current_pos.0][current_pos.1] = Tile::Sand;
            } else if grid[current_pos.0 - 1][current_pos.1 + 1] == Tile::Air {
                // fall left

                grid[current_pos.0][current_pos.1] = Tile::Air;
                current_pos.0 -= 1;
                current_pos.1 += 1;
                grid[current_pos.0][current_pos.1] = Tile::Sand;
            } else if grid[current_pos.0 + 1][current_pos.1 + 1] == Tile::Air {
                // fall right

                grid[current_pos.0][current_pos.1] = Tile::Air;
                current_pos.0 += 1;
                current_pos.1 += 1;
                grid[current_pos.0][current_pos.1] = Tile::Sand;
            } else {
                // stop

                break;
            }
        }

        if unsettled {
            break;
        } else {
            count += 1;
        }
    }

    println!("{}", count);
}

pub fn part2(inp: String) {
    let lines = inp
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|a| a.split(",").collect_tuple().unwrap())
                .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
                .collect_vec()
        })
        .collect_vec();

    let mut grid = vec![vec![Tile::Air; 1000]; 1000];

    // find the highest y value
    let floor = lines
        .iter()
        .map(|l| l.iter().map(|p| p.1).max().unwrap())
        .max()
        .unwrap()
        + 2;

    // draw the floor

    for x in 0..1000 {
        grid[x][floor as usize] = Tile::Rock;
    }

    // draw the lines

    for line in lines {
        // zip each point with the next point

        for (a, b) in line.iter().tuple_windows() {
            // draw a line between the two points

            // if the points are the same, draw a vertical line

            // println!("Drawing Line between {:?} and {:?}", a, b);

            if a.0 == b.0 {
                // draw a vertical line

                let smaller = a.1.min(b.1);
                let larger = a.1.max(b.1);

                for y in smaller..=larger {
                    grid[a.0 as usize][y as usize] = Tile::Rock;
                }
            } else {
                // draw a horizontal line

                let smaller = a.0.min(b.0);
                let larger = a.0.max(b.0);

                for x in smaller..=larger {
                    grid[x as usize][a.1 as usize] = Tile::Rock;
                }
            }
        }
    }

    let mut count = 0;

    loop {
        // spawn in a sand
        // follows the following rules
        // 1. if there is air below, fall
        // 2. if one air diagonally left fall
        // 3. if one air diagonally right fall
        // repeat until no more movement

        let mut current_pos = (500, 0);

        grid[current_pos.0][current_pos.1] = Tile::Sand;

        loop {
            // check if there is air below

            if grid[current_pos.0][current_pos.1 + 1] == Tile::Air {
                // fall

                grid[current_pos.0][current_pos.1] = Tile::Air;
                current_pos.1 += 1;
                grid[current_pos.0][current_pos.1] = Tile::Sand;
            } else if grid[current_pos.0 - 1][current_pos.1 + 1] == Tile::Air {
                // fall left

                grid[current_pos.0][current_pos.1] = Tile::Air;
                current_pos.0 -= 1;
                current_pos.1 += 1;
                grid[current_pos.0][current_pos.1] = Tile::Sand;
            } else if grid[current_pos.0 + 1][current_pos.1 + 1] == Tile::Air {
                // fall right

                grid[current_pos.0][current_pos.1] = Tile::Air;
                current_pos.0 += 1;
                current_pos.1 += 1;
                grid[current_pos.0][current_pos.1] = Tile::Sand;
            } else {
                // stop

                break;
            }
        }

        // check if the the spawn point is still sand, if so, break

        if grid[500][0] == Tile::Sand {
            break;
        } else {
            count += 1;
        }
    }

    println!("{}", count + 1);

    // printout the between 450 and 550

    for y in 0..=floor as usize {
        for x in 450..=550 {
            match grid[x][y] {
                Tile::Air => print!("."),
                Tile::Rock => print!("#"),
                Tile::Sand => print!("O"),
            }
        }

        println!();
    }
}
