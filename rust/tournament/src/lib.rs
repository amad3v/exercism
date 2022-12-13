use std::cmp::Reverse;
use std::collections::HashMap;

enum Result {
    Win,
    Draw,
    Loss,
}
#[derive(Default)]
struct Game {
    played: u8,
    won: u8,
    draw: u8,
    loss: u8,
}

impl Game {
    fn points(&self) -> u8 {
        self.draw + self.won * 3
    }

    fn update(&mut self, result: &Result) {
        self.played += 1;
        match result {
            Result::Win => self.won += 1,
            Result::Draw => self.draw += 1,
            Result::Loss => self.loss += 1,
        }
    }

    fn result(&self) -> String {
        format!(
            "|  {} |  {} |  {} |  {} |  {}",
            self.played,
            self.won,
            self.draw,
            self.loss,
            self.points()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let header = "Team                           | MP |  W |  D |  L |  P";

    if match_results.is_empty() {
        return header.to_string();
    }

    let mut teams: HashMap<&str, Game> = HashMap::new();

    match_results.split('\n').for_each(|line| {
        let mut game = line.split(';').collect::<Vec<_>>();
        let result = game.pop().unwrap();

        let pair = match result {
            "win" => (Result::Win, Result::Loss),
            "loss" => (Result::Loss, Result::Win),
            _ => (Result::Draw, Result::Draw),
        };

        teams
            .entry(game.first().unwrap())
            .or_insert_with(Game::default)
            .update(&pair.0);

        teams
            .entry(game.last().unwrap())
            .or_insert_with(Game::default)
            .update(&pair.1);
    });

    let mut teams_list = teams.iter().collect::<Vec<_>>();
    teams_list.sort_by_key(|(&team, game)| (Reverse(game.points()), team));

    format!(
        "{}\n{}",
        header,
        teams_list
            .iter()
            .map(|(team, game)| format!("{}{}{}", team, " ".repeat(31 - team.len()), game.result()))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
