use itertools::Itertools;

enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
enum Value {
    Old,
    Const(usize),
}

struct Operation {
    op: Op,
    lhs: Value,
    rhs: Value,
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    num_inspected: usize,
}

struct Test {
    test: usize,
    true_monkey: ThrowAddress,
    false_monkey: ThrowAddress,
}

#[derive(Clone, Copy)]
struct ThrowAddress(usize);

struct Throw {
    address: ThrowAddress,
    value: usize,
}

impl Operation {
    fn apply(&self, input: usize) -> usize {
        let lhs = match self.lhs {
            Value::Old => input,
            Value::Const(c) => c,
        };

        let rhs = match self.rhs {
            Value::Old => input,
            Value::Const(c) => c,
        };


        match self.op {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
        }
    }
}

impl Test {
    fn check(&self, input: usize) -> Throw {
        // mod
        if input % self.test == 0 {
            Throw {
                address: self.true_monkey,
                value: input,
            }
        } else {
            Throw {
                address: self.false_monkey,
                value: input,
            }
        }
    }
}

impl Monkey {
    fn inspect(&mut self, worry: bool) -> Throw {
        self.num_inspected += 1;
        // pop the first item
        let item = self.items.remove(0);

        // apply the operation
        let mut result = self.operation.apply(item);

        if worry {
            result /= 3
        }

        // check the test
        self.test.check(result)
    }

    fn take_turn(&mut self, worry: bool) -> Vec<Throw> {
        let mut throws = Vec::new();

        while self.items.len() > 0 {
            let throw = self.inspect(worry);
            throws.push(throw);
        }

        throws
    }

    fn catch(&mut self, throw: Throw) {
        self.items.push(throw.value);
    }
}

pub fn part1(inp: String) {
    let lines = inp.lines();

    let chunked = lines
        .chunks(7)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let monkeys = chunked.iter().map(|f| {
        let starting_items = f[1]
            .split(":")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.trim())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let operation = match f[2]
            .split(":")
            .nth(1)
            .unwrap()
            .split("=")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect_tuple()
            .unwrap()
        {
            ("old", "*", "old") => Operation {
                op: Op::Mul,
                lhs: Value::Old,
                rhs: Value::Old,
            },
            ("old", "+", "old") => Operation {
                op: Op::Add,
                lhs: Value::Old,
                rhs: Value::Old,
            },
            ("old", "*", c) => Operation {
                op: Op::Mul,
                lhs: Value::Old,
                rhs: Value::Const(c.parse::<usize>().unwrap()),
            },
            ("old", "+", c) => Operation {
                op: Op::Add,
                lhs: Value::Old,
                rhs: Value::Const(c.parse::<usize>().unwrap()),
            },
            (c, "*", "old") => Operation {
                op: Op::Mul,
                lhs: Value::Const(c.parse::<usize>().unwrap()),
                rhs: Value::Old,
            },
            (c, "+", "old") => Operation {
                op: Op::Add,
                lhs: Value::Const(c.parse::<usize>().unwrap()),
                rhs: Value::Old,
            },

            (_, _, _) => panic!("Unknown operation"),
        };

        let test: usize = f[3].split_whitespace().last().unwrap().parse().unwrap();
        let true_monkey = ThrowAddress(f[4].split_whitespace().last().unwrap().parse().unwrap());
        let false_monkey = ThrowAddress(f[5].split_whitespace().last().unwrap().parse().unwrap());

        Monkey {
            num_inspected: 0,
            items: starting_items,
            operation,
            test: Test {
                test,
                true_monkey,
                false_monkey,
            },
        }
    });

    let mut monkeys = monkeys.collect::<Vec<Monkey>>();

    // loop 20 times

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            // print out the monkey's items
            // println!("Monkey {} has {:?}", i, monkeys[i].items);
            let throws = monkeys[i].take_turn(true);

            for throw in throws {
                // println!("Monkey {} threw {} to {}", i, throw.value, throw.address.0);
                monkeys[throw.address.0 as usize].catch(throw);
            }
        }
    }

    let mut inspected = monkeys.iter().map(|m| m.num_inspected).collect_vec();

    inspected.sort();

    // print out product of top 2
    println!(
        "{}",
        inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
    );
}

pub fn part2(inp: String) {
    let lines = inp.lines();

    let chunked = lines
        .chunks(7)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let monkeys = chunked.iter().map(|f| {
        let starting_items = f[1]
            .split(":")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.trim())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let operation = match f[2]
            .split(":")
            .nth(1)
            .unwrap()
            .split("=")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect_tuple()
            .unwrap()
        {
            ("old", "*", "old") => Operation {
                op: Op::Mul,
                lhs: Value::Old,
                rhs: Value::Old,
            },
            ("old", "+", "old") => Operation {
                op: Op::Add,
                lhs: Value::Old,
                rhs: Value::Old,
            },
            ("old", "*", c) => Operation {
                op: Op::Mul,
                lhs: Value::Old,
                rhs: Value::Const(c.parse::<usize>().unwrap()),
            },
            ("old", "+", c) => Operation {
                op: Op::Add,
                lhs: Value::Old,
                rhs: Value::Const(c.parse::<usize>().unwrap()),
            },
            (c, "*", "old") => Operation {
                op: Op::Mul,
                lhs: Value::Const(c.parse::<usize>().unwrap()),
                rhs: Value::Old,
            },
            (c, "+", "old") => Operation {
                op: Op::Add,
                lhs: Value::Const(c.parse::<usize>().unwrap()),
                rhs: Value::Old,
            },

            (_, _, _) => panic!("Unknown operation"),
        };

        let test: usize = f[3].split_whitespace().last().unwrap().parse().unwrap();
        let true_monkey = ThrowAddress(f[4].split_whitespace().last().unwrap().parse().unwrap());
        let false_monkey = ThrowAddress(f[5].split_whitespace().last().unwrap().parse().unwrap());

        Monkey {
            num_inspected: 0,
            items: starting_items,
            operation,
            test: Test {
                test,
                true_monkey,
                false_monkey,
            },
        }
    });

    let common_divisor: usize = monkeys.clone().map(|v| v.test.test).product();

    let mut monkeys = monkeys.collect::<Vec<Monkey>>();

    // loop 20 times

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            // print out the monkey's items
            // println!("Monkey {} has {:?}", i, monkeys[i].items);
            let throws = monkeys[i].take_turn(false);

            for throw in throws {
                // println!("Monkey {} threw {} to {}", i, throw.value, throw.address.0);
                // modify the value to be the common divisor

                let throw = Throw {
                    address: throw.address,
                    value: throw.value % common_divisor,
                };

                monkeys[throw.address.0 as usize].catch(throw);
            }
        }
    }

    let mut inspected = monkeys.iter().map(|m| m.num_inspected).collect_vec();

    inspected.sort();

    // print out product of top 2
    println!(
        "{}",
        inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
    );
}
