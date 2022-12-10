use itertools::Itertools;

pub fn part1(inp: String) {
    let assignments = inp
        .lines()
        .map(|line| {
            let (first, second) = line
                .split(",")
                .take(2)
                .map(|f| {
                    f.split("-")
                        .map(|f| f.parse::<u32>().unwrap())
                        .collect_tuple::<(_, _)>()
                        .unwrap()
                })
                .map(|(start, end)| start..=end)
                .collect_tuple()
                .unwrap();

            // check if the first range is a subset of the second
            let first_subset = first.start() >= second.start() && first.end() <= second.end();
            // check if the second range is a subset of the first
            let second_subset = second.start() >= first.start() && second.end() <= first.end();

            first_subset || second_subset
        })
        .filter(|f| f == &true)
        .count();

    println!("{}", assignments);
}

pub fn part2(inp: String) {
    let f = inp
        .lines()
        .map(|line| {
            let (first, second) = line
                .split(",")
                .take(2)
                .map(|f| {
                    f.split("-")
                        .map(|f| f.parse::<u32>().unwrap())
                        .collect_tuple::<(_, _)>()
                        .unwrap()
                })
                .map(|(start, end)| start..=end)
                .collect_tuple()
                .unwrap();

            // check if any overlap
            first.start() <= second.end() && second.start() <= first.end()
        })
        .filter(|f| f == &true)
        .count();

    println!("{}", f);
}
