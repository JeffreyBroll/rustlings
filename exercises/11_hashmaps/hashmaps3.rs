// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team 
// conceded. One approach to build the scores table is to use a Hashmap. 
// The solution is partially written to use a Hashmap, 
// complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

impl Team {
    pub fn add(&mut self, conceded: u8, scored: u8) {
        self.goals_conceded += conceded;
        self.goals_scored += scored; 
    }
}

fn build_scores_table(results: &str) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let mut parts = r.split(',');
        let team_1_name = parts.next().unwrap().to_string();
        let team_2_name = parts.next().unwrap().to_string();
        let team_1_score: u8 = parts.next().unwrap().parse().unwrap();
        let team_2_score: u8 = parts.next().unwrap().parse().unwrap();
        
        scores
            .entry(team_1_name)
            .and_modify(|t| { t.add(team_2_score, team_1_score); })
            .or_insert(Team { goals_scored: team_1_score, goals_conceded: team_2_score });

        scores
            .entry(team_2_name)
            .and_modify(|t| { t.add(team_1_score, team_2_score); })
            .or_insert(Team { goals_scored: team_2_score, goals_conceded: team_1_score });
    };
    
    scores
}


fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
