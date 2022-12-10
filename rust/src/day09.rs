#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

use itertools::Itertools;
use Direction::*;

struct Instruction {
    dir: Direction,
    len: i32,
}

#[derive(Clone)]
struct Point {
    visited: bool,
}

#[derive(Clone)]
struct Rope {
    loc: (i32, i32),
}

pub fn part1(inp: String) {
    println!("{}", touching((0, 0), (0, 1)));
    let lines = inp.lines();

    let moves = lines
        .map(|f| f.split_whitespace().collect_tuple().unwrap())
        .map(|(d, l)| Instruction {
            dir: match d {
                "L" => Left,
                "R" => Right,
                "U" => Up,
                "D" => Down,
                _ => panic!("Invalid direction"),
            },
            len: l.parse::<i32>().unwrap(),
        })
        .collect::<Vec<Instruction>>();

    let mut visited_points = vec![];

    // head and tail both start at bottom left of grid

    let mut head = (0, 10000);
    let mut tail = (0, 10000);

    for m in moves {
        println!("{:?} {}", m.dir, m.len);
        // make the move of the head first

        let mut len = m.len;

        while len > 0 {
            // move the head one
            match m.dir {
                Left => head.0 -= 1,
                Right => head.0 += 1,
                Up => head.1 -= 1,
                Down => head.1 += 1,
            }

            len -= 1;

            println!("head {} {}", head.0, head.1);

            visited_points.push((tail.0, tail.1));
            println!("visited {} {}", tail.0, tail.1);

            if touching(head, tail) {
                println!("touching");
                continue;
            };

            // if they are one apart left right top bottom then move the tail once

            if one_apart(head, tail) {
                if head.0 == tail.0 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                } else if head.1 == tail.1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                }
            } else {
                // move diagonally to the head
                if head.0 > tail.0 {
                    tail.0 += 1;
                } else {
                    tail.0 -= 1;
                }

                if head.1 > tail.1 {
                    tail.1 += 1;
                } else {
                    tail.1 -= 1;
                }
            }
        }
    }

    println!("visited org {:?}", visited_points);

    // make it into a 2d array so its easy to visualize

    // let mut grid = vec![vec![Point { visited: false }; 9]; 9];

    // for (x, y) in visited_points {
    //     grid[y as usize][x as usize].visited = true;
    // }

    // for row in grid {
    //     for col in row {
    //         if col.visited {
    //             print!("X");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    // dedupe
    let visited = visited_points
        .iter()
        .unique()
        // .filter(|f| f.0 != 0 && f.1 != 0)
        .count();

    println!("Part 1: {}", visited + 1);
}

fn one_apart(head: (i32, i32), tail: (i32, i32)) -> bool {
    head.0 == tail.0 || head.1 == tail.1
}

fn touching(head: (i32, i32), tail: (i32, i32)) -> bool {
    // they can overlap too

    if head.0 == tail.0 && head.1 == tail.1 {
        true
    } else if head.0 == tail.0 {
        if head.1 > tail.1 {
            head.1 - tail.1 == 1
        } else {
            tail.1 - head.1 == 1
        }
    } else if head.1 == tail.1 {
        if head.0 > tail.0 {
            head.0 - tail.0 == 1
        } else {
            tail.0 - head.0 == 1
        }
    }
    // diag
    else if head.0 > tail.0 && head.1 > tail.1 {
        head.0 - tail.0 == 1 && head.1 - tail.1 == 1
    } else if head.0 < tail.0 && head.1 < tail.1 {
        tail.0 - head.0 == 1 && tail.1 - head.1 == 1
    } else if head.0 > tail.0 && head.1 < tail.1 {
        head.0 - tail.0 == 1 && tail.1 - head.1 == 1
    } else if head.0 < tail.0 && head.1 > tail.1 {
        tail.0 - head.0 == 1 && head.1 - tail.1 == 1
    } else {
        false
    }
}

pub fn part2(inp: String) {
    println!("{}", touching((0, 0), (0, 1)));
    let lines = inp.lines();

    let moves = lines
        .map(|f| f.split_whitespace().collect_tuple().unwrap())
        .map(|(d, l)| Instruction {
            dir: match d {
                "L" => Left,
                "R" => Right,
                "U" => Up,
                "D" => Down,
                _ => panic!("Invalid direction"),
            },
            len: l.parse::<i32>().unwrap(),
        })
        .collect::<Vec<Instruction>>();

    let mut visited_points = vec![];

    // head and tail both start at bottom left of grid

    // head with 9 knots
    let mut rope = vec![
        Rope { loc: (0, 100000) },
        Rope { loc: (0, 100000) },
        Rope { loc: (0, 100000) },
        Rope { loc: (0, 100000) },
        Rope { loc: (0, 100000) },
        Rope { loc: (0, 100000) },
        Rope { loc: (0, 100000) },
        Rope { loc: (0, 100000) },
        Rope { loc: (0, 100000) },
        Rope { loc: (0, 100000) },
    ];

    for m in moves {
        println!("{:?} {}", m.dir, m.len);
        // make the move of the head first

        let mut len = m.len;

        while len > 0 {
            // move the head one
            let head = &mut rope[0];

            match m.dir {
                Left => head.loc.0 -= 1,
                Right => head.loc.0 += 1,
                Up => head.loc.1 -= 1,
                Down => head.loc.1 += 1,
            }

            len -= 1;

            // println!("head {} {}", head.0, head.1);

            for idx in 1..10 {
                let head = rope[idx - 1].clone();
                let tail = &mut rope[idx];

                if touching(head.loc, tail.loc) {
                    println!("touching");
                    continue;
                };

                // if they are one apart left right top bottom then move the tail once

                if one_apart(head.loc, tail.loc) {
                    if head.loc.0 == tail.loc.0 {
                        if head.loc.1 > tail.loc.1 {
                            tail.loc.1 += 1;
                        } else {
                            tail.loc.1 -= 1;
                        }
                    } else if head.loc.1 == tail.loc.1 {
                        if head.loc.0 > tail.loc.0 {
                            tail.loc.0 += 1;
                        } else {
                            tail.loc.0 -= 1;
                        }
                    }
                } else {
                    // move diagonally to the head
                    if head.loc.0 > tail.loc.0 {
                        tail.loc.0 += 1;
                    } else {
                        tail.loc.0 -= 1;
                    }

                    if head.loc.1 > tail.loc.1 {
                        tail.loc.1 += 1;
                    } else {
                        tail.loc.1 -= 1;
                    }
                }
            }

            let last = rope.last().unwrap();

            visited_points.push((last.loc.0, last.loc.1));
        }
    }

    println!("visited org {:?}", visited_points);

    // make it into a 2d array so its easy to visualize

    // let mut grid = vec![vec![Point { visited: false }; 9]; 9];

    // for (x, y) in visited_points {
    //     grid[y as usize][x as usize].visited = true;
    // }

    // for row in grid {
    //     for col in row {
    //         if col.visited {
    //             print!("X");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    // dedupe
    let visited = visited_points
        .iter()
        .unique()
        // .filter(|f| f.0 != 0 && f.1 != 0)
        .count();

    println!("Part 1: {}", visited);
}
