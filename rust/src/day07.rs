#[derive(Debug)]
enum Node {
    Directory(Directory),
    File(File),
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Node::Directory(d) => d.name.as_str(),
            Node::File(f) => f.name.as_str(),
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<Node>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
enum Line {
    Input { command: String, args: String },
    Output(FileKind),
}

#[derive(Debug)]
enum FileKind {
    Dir(String),
    File(usize, String),
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            children: vec![],
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn get_child(&mut self, name: &str) -> Option<&mut Node> {
        self.children.iter_mut().find(|f| match f {
            Node::Directory(d) => d.name == name,
            Node::File(f) => f.name == name,
        })
    }

    fn add_child_if_not_exists(&mut self, child: Node) {
        if self.get_child(child.name()).is_none() {
            self.add_child(child);
        }
    }

    fn size(&self) -> usize {
        self.children
            .iter()
            .map(|f| match f {
                Node::Directory(d) => d.size(),
                Node::File(f) => f.size,
            })
            .sum()
    }
}

fn parse_line(line: String) -> Line {
    // split it

    let split = line.split(" ").collect::<Vec<&str>>();

    // if it starts with $ then its a command else its an output

    match (split[0], split[1]) {
        // match the first word ls or cd
        ("$", "cd") => Line::Input {
            command: "cd".to_string(),
            args: split[2].to_string(),
        },
        ("$", "ls") => Line::Input {
            command: "ls".to_string(),
            args: "".to_string(),
        },

        ("dir", x) => Line::Output(FileKind::Dir(x.to_string())),
        (size, name) => Line::Output(FileKind::File(
            size.parse::<usize>().unwrap(),
            name.to_string(),
        )),
    }
}

fn get_dir(root: &mut Directory, path: Vec<String>) -> &mut Directory {
    let mut dir = root;

    for name in path {
        if name == "/" {
            continue;
        }

        dir = match dir.get_child(name.as_str()) {
            Some(Node::Directory(d)) => d,
            _ => panic!("Directory not found"),
        };
    }

    dir
}

fn create_tree(inp: String) -> Directory {
    let lines = inp
        .lines()
        .map(|f| f.to_string())
        .map(|f| parse_line(f))
        .collect::<Vec<Line>>();

    let mut root = Directory {
        name: "/".to_string(),
        children: vec![],
    };

    let mut current_dir = vec![root.name.clone()];

    for line in lines {
        // print out the line in bold with white background
        // println!("\x1b[1;37m{:?}\x1b[0m", line);
        // println!("Current dir: {:?}", current_dir);
        // println!("Root: {:?}", root);

        match line {
            Line::Input { command, args } => {
                match command.as_str() {
                    "cd" => {
                        match args.as_str() {
                            ".." => {
                                current_dir.pop();
                            }
                            "/" => {
                                current_dir = vec!["/".to_string()];
                            }
                            _ => {
                                // create the directory if it doesn't exist
                                let dir = get_dir(&mut root, current_dir.clone());

                                dir.add_child_if_not_exists(Node::Directory(Directory::new(
                                    args.clone(),
                                )));

                                current_dir.push(args.clone());
                            }
                        }
                    }
                    "ls" => {}
                    _ => panic!("Unknown command"),
                }
            }

            Line::Output(file) => match file {
                FileKind::Dir(name) => {
                    // println!("Adding dir: {}", name);
                    let dir = get_dir(&mut root, current_dir.clone());

                    dir.add_child_if_not_exists(Node::Directory(Directory::new(name)));
                }
                FileKind::File(size, name) => {
                    // println!("Adding file: {} {}", size, name);
                    let dir = get_dir(&mut root, current_dir.clone());

                    dir.add_child_if_not_exists(Node::File(File { name, size }));
                }
            },
        }
    }

    root
}

pub fn part1(inp: String) {
    // get a list of all the nested directories
    // breadth first first

    let root = create_tree(inp);

    let mut queue = vec![root];
    let mut dirs = vec![];

    while !queue.is_empty() {
        let dir = queue.remove(0);

        dirs.push(dir.size());

        for child in dir.children {
            match child {
                Node::Directory(d) => queue.push(d),
                _ => {}
            }
        }
    }

    let sizes = dirs.iter().filter(|f| **f <= 100000).sum::<usize>();

    println!("Total size: {:?}", sizes);
}

pub fn part2(inp: String) {
    let root = create_tree(inp);

    let root_size = root.size();

    let mut queue = vec![root];
    let mut dirs = vec![];

    while !queue.is_empty() {
        let dir = queue.remove(0);

        dirs.push(dir.size());

        for child in dir.children {
            match child {
                Node::Directory(d) => queue.push(d),
                _ => {}
            }
        }
    }

    let space_needed = root_size - (70000000 - 30000000);

    let sizes = dirs.iter().filter(|f| **f >= space_needed).min().unwrap();

    println!("{:?}", sizes)
}
