use std::collections::HashMap;

#[derive(Debug)]
struct Valve {
    flow_rate: i32,
    tunnels: Vec<String>,
}

pub fn part1(inp: String) {
    let mut lines = inp
        .lines()
        .map(|f| {
            // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB

            let mut parts = f.split_whitespace();

            // name should be index 1

            let name = parts.nth(1).unwrap().to_string();

            // flow rate should be index 4, but we need to strip the prefix rate= and the suffix ;

            let flow_rate = parts
                .nth(2)
                .unwrap()
                .strip_prefix("rate=")
                .unwrap()
                .strip_suffix(";")
                .unwrap()
                .parse::<i32>()
                .unwrap();

            // tunnels will be anything after index 7

            let tunnels = parts.skip(4).map(|f| f.to_string()).collect::<Vec<_>>();

            (name, Valve { flow_rate, tunnels })
        })
        .collect::<HashMap<_, _>>();

    println!("{:?}", lines);
}
