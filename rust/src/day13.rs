use itertools::Itertools;

#[derive(Debug, Clone)]
enum Value {
    Literal(u32),
    List(Vec<Value>),
}

#[derive(Eq, PartialEq, Debug)]
enum Equality {
    LessThan,
    GreaterThan,
    Equal,
}

impl Value {
    fn parse(input: &str) -> (Value, &str) {
        // use a match statement, if its [] then its a list, otherwise its a literal
        // literals will be a single number

        // if its a list, then we need to parse the list
        // if its a literal, then we need to parse the literal

        let mut chars = input.chars();
        let first = chars.next().unwrap();

        match first {
            '[' => {
                // should be able to parse the following:
                // [1,2,3]
                // [1,2,3,4,5,6,7,8,9,10]
                // [1,[2,[3,[4,[5,6,7]]]],8,9]
                // [[[]]]

                let mut list = Vec::new();
                let mut rest = chars.as_str();

                loop {
                    // make sure ] is not the first character
                    let mut chars = rest.chars();
                    let first = chars.next().unwrap();
                    if first == ']' {
                        rest = chars.as_str();
                        break;
                    }

                    let (val, new_rest) = Value::parse(rest);
                    list.push(val);
                    rest = new_rest;

                    let mut chars = rest.chars();
                    let first = chars.next().unwrap();
                    if first == ',' {
                        rest = chars.as_str();
                    } else if first == ']' {
                        rest = chars.as_str();
                        break;
                    } else {
                        panic!("unexpected character {}", first);
                    }
                }

                (Value::List(list), rest)
            }

            _ => {
                let mut num = String::new();
                let mut rest = input;
                loop {
                    let mut chars = rest.chars();
                    let first = chars.next().unwrap();
                    if first.is_numeric() {
                        num.push(first);
                        rest = chars.as_str();
                    } else {
                        break;
                    }
                }
                (Value::Literal(num.parse().unwrap()), rest)
            }
        }
    }

    fn compare(&self, other: Self) -> Equality {
        let res = match (self.clone(), other) {
            (Value::Literal(l), Value::Literal(r)) => {
                if l < r {
                    Equality::LessThan
                } else if l > r {
                    Equality::GreaterThan
                } else {
                    Equality::Equal
                }
            }
            (Value::Literal(l), Value::List(r)) => {
                // create a new comparison and recurse
                Value::List(vec![Value::Literal(l)]).compare(Value::List(r))
            }
            (Value::List(l), Value::Literal(r)) => {
                // create a new comparison and recurse

                self.compare(Value::List(vec![Value::Literal(r)]))
            }

            (Value::List(l), Value::List(r)) => {
                // the following rules apply:
                // 1. start comparing item by item
                // 2. if the items are equal, then move on to the next item
                // 3. if the items are not equal, then return the result of the comparison
                // 4. if we run out of items on one side, then the other side is greater

                let mut l_iter = l.iter();
                let mut r_iter = r.iter();

                loop {
                    let l_item = l_iter.next();
                    let r_item = r_iter.next();

                    match (l_item, r_item) {
                        (Some(l), Some(r)) => {
                            let result = l.compare(r.clone());

                            if result != Equality::Equal {
                                return result;
                            }
                        }
                        (Some(_), None) => return Equality::GreaterThan,
                        (None, Some(_)) => return Equality::LessThan,
                        (None, None) => return Equality::Equal,
                    }
                }
            }
        };

        res
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.compare(other.clone()) {
            Equality::LessThan => std::cmp::Ordering::Less,
            Equality::GreaterThan => std::cmp::Ordering::Greater,
            Equality::Equal => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self.compare(other.clone()) {
            Equality::Equal => true,
            _ => false,
        }
    }
}

impl Eq for Value {}

#[derive(Debug)]
struct Comparison {
    lhs: Value,
    rhs: Value,
}

pub fn part1(inp: String) {
    let lines = inp.lines().collect::<Vec<_>>();

    let lines = lines.chunks(3).map(|w| {
        let left = Value::parse(w[0]);
        let right = Value::parse(w[1]);

        Comparison {
            lhs: left.0,
            rhs: right.0,
        }
    });

    let results = lines.map(|c| {
        match c.lhs.compare(c.rhs) {
            // if the left is less than the right then return true
            Equality::LessThan => true,
            _ => false,
        }
    });

    let results: usize = results
        .enumerate()
        .filter(|(_, r)| *r)
        .map(|(i, _)| i + 1)
        .sum();

    println!("{:?}", results);
}

// left should always be lower than right
// run out of items first

pub fn part2(inp: String) {
    let mut lines = inp
        .lines()
        .map(|f| {
            if f != "" {
                Some(Value::parse(f).0)
            } else {
                None
            }
        })
        // remove the None values and remove the Some wrapper
        .filter(|f| f.is_some())
        .map(|f| f.unwrap())
        .collect::<Vec<_>>();

    // add on [[2]] and [[6]]

    let dividers = vec![Value::parse("[[2]]").0, Value::parse("[[6]]").0];

    lines.extend(dividers);

    lines.sort();

    // get the indexes of the dividers
    let dividers: usize = lines
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == Value::parse("[[2]]").0 || **v == Value::parse("[[6]]").0)
        .map(|(i, _)| i + 1)
        .product();

    println!("{:?}", dividers);
}
