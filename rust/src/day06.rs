use itertools::Itertools;

pub fn part1(inp: String) {
    // iterate till you find the first 4 unique chars
    // then you know the index

    let chars = inp.chars().collect::<Vec<char>>();

    // iterate over chunks of 4
    for i in 0..chars.len() {
        let chunk = chars[i..i + 4].to_vec();

        // if the chunk is unique, then we found the index
        if chunk.iter().all_unique() {
            println!("{:?}", chunk);
            println!("{}", i + 4);
            break;
        }
    }
}

pub fn part2(inp: String) {
    let chars = inp.chars().collect::<Vec<char>>();

    // iterate over chunks of 4
    for i in 0..chars.len() {
        let chunk = chars[i..i + 14].to_vec();

        // if the chunk is unique, then we found the index
        if chunk.iter().all_unique() {
            println!("{:?}", chunk);
            println!("{}", i + 14);
            break;
        }
    }
}
