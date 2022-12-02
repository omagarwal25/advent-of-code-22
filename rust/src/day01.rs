pub fn part1(inp: String) {
    println!("{}", elves(inp).iter().max().unwrap());
}

pub fn part2(inp: String) {
    let mut elves = elves(inp);

    elves.sort();

    println!("{}", elves.iter().rev().take(3).sum::<i32>());
}

fn elves(inp: String) -> Vec<i32> {
    let split = inp
        .split('\n')
        .map(|x| x.parse::<i32>().map_or(None, |x| Some(x)))
        .collect::<Vec<Option<i32>>>();

    let mut elves: Vec<i32> = vec![];

    for i in split {
        if let Some(i) = i {
            // if the last element is None, push on i, otherwise add to it use vec.last()
            if let Some(element) = elves.last_mut() {
                // add to the last element
                *element += i;
            } else {
                elves.push(i);
            }
        } else {
            elves.push(0);
        }
    }

    elves
}
