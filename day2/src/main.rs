#[derive(Debug, PartialEq, Eq, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

impl std::str::FromStr for Choice {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

impl Choice {
    fn get_points(&self) -> usize {
        match self {
            &Choice::Rock => 1,
            &Choice::Paper => 2,
            &Choice::Scissors => 3,
        }
    }

    fn winning_points(&self, other: &Self) -> usize {
        match (self, other) {
            (&Choice::Paper, &Choice::Rock) => 6,
            (&Choice::Rock, &Choice::Scissors) => 6,
            (&Choice::Scissors, &Choice::Paper) => 6,
            (me, them) if me == them => 3,
            _ => 0
        }
    }

    fn win_choice(&self) -> Self {
        match self {
            &Choice::Rock => Choice::Paper,
            &Choice::Paper => Choice::Scissors,
            &Choice::Scissors => Choice::Rock,
        }
    }

    fn lose_choice(&self) -> Self {
        match self {
            &Choice::Rock => Choice::Scissors,
            &Choice::Paper => Choice::Rock,
            &Choice::Scissors => Choice::Paper,
        }
    }

    fn tie_choice(&self) -> Self {
        self.clone()
    }
}

fn choices_from_line(line: &str) -> (Choice, Choice) {
    let mut choices = line.split_whitespace();
    (
        choices.next().unwrap().parse::<Choice>().unwrap(),
        choices.next().unwrap().parse::<Choice>().unwrap(),
    )
}

fn choices_from_line2(line: &str) -> (Choice, Choice) {
    let mut choices = line.split_whitespace();
    let them = choices.next().unwrap().parse::<Choice>().unwrap();
    (
        them.clone(),
        match choices.next().unwrap() {
            "X" => them.lose_choice(),
            "Y" => them.tie_choice(),
            "Z" => them.win_choice(),
            _ => panic!("Wrong choice type"),
        }
    )
}

fn main() {
    let input = std::fs::read_to_string("day2.txt").unwrap();
    let points: usize = input.split_terminator('\n')
        .map(|line| choices_from_line(line))
        .map(|(them, me)| me.get_points() + me.winning_points(&them))
        .sum();
    println!("points: {points}");
}
