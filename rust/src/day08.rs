use itertools::Itertools;

struct Tree {
    size: i32,
    seen: bool,
}

pub fn part1(inp: String) {
    let mut grid = inp
        .lines()
        .map(|f| {
            f.chars()
                .map(|c| c.to_digit(10).unwrap())
                .map(|f| Tree {
                    size: f as i32,
                    seen: false,
                })
                .collect_vec()
        })
        .collect_vec();

    for i in 0..grid.len() {
        let mut curr_highest: i32 = -100;
        for j in 0..grid[i].len() {
            if grid[i][j].size > curr_highest {
                curr_highest = grid[i][j].size;

                grid[i][j].seen = true;
            }
        }
    }

    for i in 0..grid[0].len() {
        let mut curr_highest: i32 = -100;

        for j in (0..grid.len()) {
            if grid[j][i].size > curr_highest {
                curr_highest = grid[j][i].size;

                grid[j][i].seen = true;
            }
        }
    }

    for i in 0..grid.len() {
        let mut curr_highest: i32 = -100;

        for j in (0..grid[i].len()).rev() {
            if grid[i][j].size > curr_highest {
                curr_highest = grid[i][j].size;

                grid[i][j].seen = true;
            }
        }
    }

    for i in 0..grid[0].len() {
        let mut curr_highest: i32 = -100;

        for j in (0..grid.len()).rev() {
            if grid[j][i].size > curr_highest {
                curr_highest = grid[j][i].size;

                grid[j][i].seen = true;
            }
        }
    }

    let sum = grid
        .iter()
        .map(|f| f.iter().filter(|f| f.seen).count())
        .sum::<usize>();

    println!("Sum: {:?}", sum);
}

pub fn part2(inp: String) {
    let grid = inp
        .lines()
        .map(|f| {
            f.chars()
                .map(|c| c.to_digit(10).unwrap())
                .map(|f| Tree {
                    size: f as i32,
                    seen: false,
                })
                .collect_vec()
        })
        .collect_vec();

    let mut max = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // starting at this tree go in each cardinal direction until you find a tree with the same size or a tree with a higher size, total the amount of trees you find in each direction

            let size = grid[i][j].size;

            let up = (0..i)
                .rev()
                .map(|f| grid[f][j].size)
                .take_while(|f| *f < size)
                .count();

            let up = if up == i { up } else { up + 1 };

            let down = (i + 1..grid.len())
                .map(|f| grid[f][j].size)
                .take_while(|f| *f < size)
                .count();

            let down = if down == grid.len() - (i + 1) {
                down
            } else {
                down + 1
            };

            let left = (0..j)
                .rev()
                .map(|f| grid[i][f].size)
                .take_while(|f| *f < size)
                .count();

            let left = if left == j { left } else { left + 1 };

            let right = (j + 1..grid[i].len())
                .map(|f| grid[i][f].size)
                .take_while(|f| *f < size)
                .count();

            // println!("{}", grid[i]);

            let right = if right == grid[i].len() - (j + 1) {
                right
            } else {
                right + 1
            };

            max = max.max(up * down * left * right);
        }
    }

    println!("Max: {:?}", max);
}
