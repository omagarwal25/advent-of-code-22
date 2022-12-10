use itertools::Itertools;

enum Instruction {
    Addx(i32),
    Noop,
}

pub fn part1(inp: String) {
    let lines = inp.lines().map(|f| {
        if f == "noop" {
            Instruction::Noop
        } else {
            let (_, v) = f.split_whitespace().collect_tuple().unwrap();
            Instruction::Addx(v.parse::<i32>().unwrap())
        }
    });

    let mut cycles = 0;
    let mut v = 1;

    let mut values = vec![];

    let times = vec![20, 60, 100, 140, 180, 220];

    for l in lines {
        cycles += 1;

        if times.contains(&cycles) {
            values.push(v * cycles);
            println!("{} {}", v, cycles)
        }

        match l {
            Instruction::Addx(x) => {
                cycles += 1;

                if times.contains(&cycles) {
                    values.push(v * cycles);
                    println!("{} {}", v, cycles)
                }

                v += x;
            }
            Instruction::Noop => (),
        }
    }

    println!("{:?}", values.iter().sum::<i32>());
}

pub fn part2(inp: String) {
    let lines = inp.lines().map(|f| {
        if f == "noop" {
            Instruction::Noop
        } else {
            let (_, v) = f.split_whitespace().collect_tuple().unwrap();
            Instruction::Addx(v.parse::<i32>().unwrap())
        }
    });

    let mut cycles = 0;
    let mut v: i32 = 1;

    // let mut values = vec![];
    let mut screen = vec![];

    let times = vec![20, 60, 100, 140, 180, 220];

    for l in lines {
        cycles += 1;

        // if times.contains(&cycles) {
        //     // values.push(v * cycles);
        //     // println!("{} {}", v, cycles)
        // }

        screen.push(((v - 1)..=(v + 1)).contains(&((screen.len() % 40) as i32)));

        match l {
            Instruction::Addx(x) => {
                cycles += 1;

                // if times.contains(&cycles) {
                //     // values.push(v * cycles);
                //     // println!("{} {}", v, cycles)
                // }

                screen.push(((v - 1)..=(v + 1)).contains(&((screen.len() % 40) as i32)));
                v += x;
            }
            Instruction::Noop => (),
        }
    }

    let mapped = screen
        .iter()
        .map(|f| if *f { "#" } else { "." })
        .collect_vec();

    println!("{}", mapped.len());

    // print out the screen 6 high by 40 wide

    for i in 0..6 {
        println!("{}", mapped.iter().skip(i * 40).take(40).join(""));
    }
}
