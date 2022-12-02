#[derive(Clone, PartialEq)]
enum GameResult {
    Win,
    Loss,
    Draw,
}

impl From<char> for GameResult {
    fn from(c: char) -> Self {
        match c {
            'Z' => GameResult::Win,
            'X' => GameResult::Loss,
            'Y' => GameResult::Draw,
            _ => panic!("Invalid game result: {}", c),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn win_against(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn lose_against(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn tie_against(&self) -> Move {
        self.clone()
    }

    fn beats(&self, other: &Move) -> bool {
        match (self, other) {
            (Move::Rock, Move::Scissors) => true,
            (Move::Paper, Move::Rock) => true,
            (Move::Scissors, Move::Paper) => true,
            _ => false,
        }
    }

    fn from_opponent_char(c: char) -> Option<Move> {
        match c {
            'A' => Some(Move::Rock),
            'B' => Some(Move::Paper),
            'C' => Some(Move::Scissors),
            _ => None,
        }
    }

    fn from_player_char(c: char) -> Option<Move> {
        match c {
            'X' => Some(Move::Rock),
            'Y' => Some(Move::Paper),
            'Z' => Some(Move::Scissors),
            _ => None,
        }
    }
}

pub fn part1(inp: String) {
    let result = inp
        .lines()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .take(2)
                .map(|f| f.chars().next().unwrap())
                .collect::<Vec<char>>()
        })
        .map(|line| {
            let player = Move::from_player_char(line[1]).unwrap();
            let opponent = Move::from_opponent_char(line[0]).unwrap();

            (player, opponent)
        })
        .map(|(player, opponent)| {
            player.score()
                + if player.beats(&opponent) {
                    6
                } else if opponent == player {
                    3
                } else {
                    0
                }
        })
        .sum::<i32>();

    println!("{}", result);
}

pub fn part2(inp: String) {
    let result = inp
        .lines()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .take(2)
                .map(|f| f.chars().next().unwrap())
                .collect::<Vec<char>>()
        })
        .map(|line| {
            let result = GameResult::from(line[1]);
            let opponent = Move::from_opponent_char(line[0]).unwrap();

            (result, opponent)
        })
        .map(|(result, opponent)| {
            if result == GameResult::Win {
                opponent.win_against().score() + 6
            } else if result == GameResult::Loss {
                opponent.lose_against().score() + 0
            } else {
                opponent.tie_against().score() + 3
            }
        })
        .sum::<i32>();

    println!("{}", result);
}
