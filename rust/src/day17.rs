use itertools::Itertools;

enum Dir {
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Air,
    Block,
    Falling,
}

pub fn part1(inp: String) {
    //####

    // .#.
    // ###
    // .#.

    // ..#
    // ..#
    // ###

    // #
    // #
    // #
    // #

    // ##
    // ##

    // upside down because we are flipping the grid
    let blocks = vec![
        vec![vec![Tile::Block, Tile::Block, Tile::Block]],
        vec![
            vec![Tile::Air, Tile::Block, Tile::Air],
            vec![Tile::Block, Tile::Block, Tile::Block],
            vec![Tile::Air, Tile::Block, Tile::Air],
        ],
        vec![
            vec![Tile::Block, Tile::Block, Tile::Block],
            vec![Tile::Block, Tile::Air, Tile::Air],
            vec![Tile::Block, Tile::Air, Tile::Air],
        ],
        vec![
            vec![Tile::Block],
            vec![Tile::Block],
            vec![Tile::Block],
            vec![Tile::Block],
        ],
        vec![
            vec![Tile::Block, Tile::Block],
            vec![Tile::Block, Tile::Block],
        ],
    ];

    let stream = inp.lines().next().unwrap().chars().map(|c| match c {
        '<' => Dir::Left,
        '>' => Dir::Right,
        _ => panic!("unexpected char"),
    });

    // okay we have a infinitly high 7 wide grid, so we will actually flip it

    let mut grid = vec![vec![Tile::Air; 7]; 10000];

    // okay so we loop over the stream and the 7 block types
    // i want to use iterators to do this

    let mut air_stream = stream.clone().cycle();
    let mut block_stream = blocks.iter().cycle();

    for _ in 0..=2022 {
        let current_block = block_stream.next().unwrap();

        // so it will alternative between getting pushed to the left and right and then falling

        // we also needs to detect if we can move any further down

        // spawn in the current block

        let highest_block = get_highest_block(&grid);

        // the lowest point is the highest block + 3

        let lowest_point = highest_block + 3;

        // it spawns in the middle of the grid

        


        while let Some(dir) = air_stream.next() {
            
        }
    }
}

fn get_highest_block(grid: &Vec<Vec<Tile>>) -> usize {
    // highest will have the highest index
    let mut highest = 0;

    for (y, row) in grid.iter().enumerate() {
        for tile in row.iter() {
            if *tile == Tile::Block {
                highest = y;
            }
        }
    }

    highest
}
