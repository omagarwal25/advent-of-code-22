use std::collections::HashSet;

pub fn part1(inp: String) {
    // get all the lines split each line into first and second half, then find the common charecters between the halves
    let lines = inp
        .lines()
        .map(|x| x.split_at(x.len() / 2))
        .collect::<Vec<(&str, &str)>>();

    let chars: Vec<char> = lines
        .iter()
        .map(|(x, y)| {
            // get the common charecters between the two halves, in any potion using
            // the intersection method

            println!("x: {}, y: {}", x, y);

            x.chars()
                .collect::<HashSet<_>>()
                .intersection(&y.chars().collect::<HashSet<_>>())
                .next()
                .unwrap()
                .clone()
        })
        .collect();

    // convert all the chars to ints a-z = 1-26 and A-Z = 27-52

    let mut sum: u32 = 0;
    for (i, c) in chars.iter().enumerate() {
        let val = match c {
            'a'..='z' => *c as u8 - 96,
            'A'..='Z' => *c as u8 - 38,
            _ => 0,
        };

        sum += val as u32
    }

    println!("{}", sum);
}

pub fn part2(inp: String) {
    let lines = inp.lines().collect::<Vec<&str>>();

    // chunk the lines in groups of 3

    let chunks = lines.chunks(3);

    let intersections = chunks
        .map(|chunk| {
            chunk[0]
                .chars()
                .collect::<HashSet<_>>()
                .intersection(&chunk[1].chars().collect::<HashSet<_>>())
                .map(|f| f.clone())
                .collect::<HashSet<char>>()
                .clone()
                .intersection(&chunk[2].chars().collect::<HashSet<_>>())
                .next()
                .unwrap()
                .clone()
                .clone()
        })
        .collect::<Vec<char>>();

    let sum = intersections
        .iter()
        .map(|c| match c {
            'a'..='z' => *c as u8 - 96,
            'A'..='Z' => *c as u8 - 38,
            _ => 0,
        } as u32)
        .sum::<u32>();

    println!("{}", sum);
}
