use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use crossterm::{execute,
                terminal::{ClearType, Clear, SetTitle},
                style::{SetForegroundColor, Color},
};


fn main() {
    // Attempt to read the file, or throw an error if it fails
    match File::open("questions.json") {
        Ok(file) => {
            // Create a buffered reader to read the file efficiently
            let reader = BufReader::new(file);

            // Parse the JSON file into the Questionnaire struct
            // Give the user feedback if it is formatted incorrectly
            let questionnaire: Questionnaire = match serde_json::from_reader(reader) {
                Ok(questionnaire) => questionnaire,
                Err(error) => {
                    println!("Error: {:?}", error);
                    return;
                }
            };

            let mut keirsey = Keirsey::new(questionnaire);

            keirsey.ask_questions();
            keirsey.print_temperament();

            set_color(Color::Yellow);
            println!("\nWould you like to take the test again? (Y/N)");
            set_color(Color::Green);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            if input.trim().to_uppercase() == "Y" {
                main();
            }
        }
        Err(error) => println!("Error: {:?}", error),
    }
}

fn welcome_the_avatar() {
    set_color(Color::Cyan);
    println!("Water. Earth. Fire. Air.\n Long ago, the four temperaments thrived in harmony. Then, everything changed when the Rational Nation attacked.\n Only the Avatar, master of all four temperaments, could bring balance, but when the world needed them most, they vanished.\n A hundred years passed, and my sibling and I discovered the new Avatar, an idealist named Keirsey.");
    set_color(Color::Reset);
}

#[derive(Debug, Deserialize, Clone)]
struct Keirsey {
    questionnaire: Questionnaire,
    answer_grid: ScoringGrid,
    temperaments: Temperaments,
}

impl Keirsey {
    fn new(questionnaire:Questionnaire) -> Keirsey {
        Keirsey {
            questionnaire,
            answer_grid: ScoringGrid::new(),
            temperaments: serde_json::from_reader(BufReader::new(File::open("temperaments.json").unwrap())).unwrap()
        }
    }
    fn ask_questions(&mut self) {
        //Clear the terminal and set the title
        execute!(std::io::stdout(), Clear(ClearType::All), SetTitle("Keirsey Temperament Sorter")).unwrap();
        for (i, question) in self.questionnaire.questions.iter().enumerate() {
            println!("Question {} of {}", i+1, self.questionnaire.questions.len());
            let answer = question.ask();
            self.answer_grid.add_score(Score::new(answer));
            clear_terminal();
        }
    }
    fn print_temperament(&self) {
        let temperament = self.answer_grid.get_temperament();

        set_color(Color::White);
        println!("Your temperament is: ");
        set_color(Color::Green);
        print!("{}\n", temperament);
        set_color(Color::Reset);

        match temperament.as_str() {
            "ISFP" => {self.temperaments.print(TemperamentType::Artisan)},
            "XSFP" => {self.temperaments.print(TemperamentType::Artisan)},
            "XSTP" => {self.temperaments.print(TemperamentType::Artisan)},
            "XSXP" => {self.temperaments.print(TemperamentType::Artisan)},
            "ESXP" => {self.temperaments.print(TemperamentType::Artisan)},
            "ISTP" => {self.temperaments.print(TemperamentType::Artisan)},
            "ISXP" => {self.temperaments.print(TemperamentType::Artisan)},
            "ESFP" => {self.temperaments.print(TemperamentType::Artisan)},
            "ESTP" => {self.temperaments.print(TemperamentType::Artisan)},

            "INFP" => {self.temperaments.print(TemperamentType::Idealist)},
            "INFJ" => {self.temperaments.print(TemperamentType::Idealist)},
            "ENFP" => {self.temperaments.print(TemperamentType::Idealist)},
            "ENFJ" => {self.temperaments.print(TemperamentType::Idealist)},
            "XNFX" => {self.temperaments.print(TemperamentType::Idealist)},
            "ENFX" => {self.temperaments.print(TemperamentType::Idealist)},
            "INFX" => {self.temperaments.print(TemperamentType::Idealist)},
            "XNFJ" => {self.temperaments.print(TemperamentType::Idealist)},
            "XNFP" => {self.temperaments.print(TemperamentType::Idealist)},

            "INTP" => {self.temperaments.print(TemperamentType::Rational)},
            "INTJ" => {self.temperaments.print(TemperamentType::Rational)},
            "ENTP" => {self.temperaments.print(TemperamentType::Rational)},
            "ENTJ" => {self.temperaments.print(TemperamentType::Rational)},
            "XNTX" => {self.temperaments.print(TemperamentType::Rational)},
            "ENTX" => {self.temperaments.print(TemperamentType::Rational)},
            "INTX" => {self.temperaments.print(TemperamentType::Rational)},
            "XNTJ" => {self.temperaments.print(TemperamentType::Rational)},
            "XNTP" => {self.temperaments.print(TemperamentType::Rational)},

            "ISFJ" => {self.temperaments.print(TemperamentType::Guardian)},
            "XSFJ" => {self.temperaments.print(TemperamentType::Guardian)},
            "ISXJ" => {self.temperaments.print(TemperamentType::Guardian)},
            "ESXJ" => {self.temperaments.print(TemperamentType::Guardian)},
            "XSXJ" => {self.temperaments.print(TemperamentType::Guardian)},
            "XSTJ" => {self.temperaments.print(TemperamentType::Guardian)},
            "ESFJ" => {self.temperaments.print(TemperamentType::Guardian)},
            "ESTJ" => {self.temperaments.print(TemperamentType::Guardian)},
            "ISTJ" => {self.temperaments.print(TemperamentType::Guardian)},

            "XXXX" => { welcome_the_avatar()}
            _ => {
                println!("Your Temperament is balanced and you do not fall into any category.");
                self.temperaments.print(TemperamentType::Artisan);
                self.temperaments.print(TemperamentType::Guardian);
                self.temperaments.print(TemperamentType::Idealist);
                self.temperaments.print(TemperamentType::Rational);
            }
        }
    }
}


#[derive(Debug, Deserialize, Clone)]
struct Questionnaire {
    questions: Vec<Question>,
}
#[derive(Debug, Deserialize, Clone)]
struct Question {
    question: String,
    options: Vec<Options>,
}

impl Question {
    fn ask(&self) -> Answer {

        set_color(Color::Green);
        println!("\n{}", self.question);

        set_color(Color::White);
        for option in &self.options {
            match option {
                Options::A(text) => println!("A: {}", text),
                Options::B(text) => println!("B: {}", text),
            }
        }

        set_color(Color::Cyan);
        // Await input from the user for the answer
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_uppercase() == "A" {
            return Answer::A;
        } else if input.trim().to_uppercase() == "B" {
            return Answer::B;
        } else {
            clear_terminal();
            set_color(Color::Red);
            print!("Invalid input");
            set_color(Color::Reset);
            self.ask()
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Score {
    value: Answer,
}
impl Score {
    fn new(value: Answer) -> Score {
        Score {
            value,
        }
    }

}

#[derive(Debug, Deserialize, Clone)]
struct ScoringGrid {
    scores: Vec<Vec<Score>>,
}
impl ScoringGrid {
    fn new() -> ScoringGrid {
        ScoringGrid {
            scores: Vec::new(),
        }
    }
    fn add_score(&mut self, score: Score) {
        let score_array_length = self.scores.len();

        if score_array_length == 0 {
            self.scores.push(Vec::new());
            self.scores[score_array_length].push(score)
        } else if self.scores[score_array_length-1].len() % 7 == 0 {
            self.scores.push(Vec::new());
            self.scores[score_array_length].push(score);
        } else {
            self.scores[score_array_length-1].push(score);
        }
    }

    fn tally(&self) -> Vec<(u8, u8)> {
        let mut groups:Vec<(u8, u8)> = vec![];

        let mut i = 0;
        while i < 7 {
            let mut count_a:u8 = 0;
            let mut count_b:u8 = 0;
            for scores in &self.scores {

                match scores[i].value {
                    Answer::A => count_a += 1,
                    Answer::B => count_b += 1,
                }
            };
            match i {
                2 => {
                    groups.push((count_a, count_b));
                    groups.push((count_a+ groups[1].0, count_b + groups[1].1));
                },
                4 => {
                    groups.push((count_a, count_b));
                    groups.push((count_a + groups[4].0, count_b + groups[4].1));
                },
                6 => {
                    groups.push((count_a, count_b));
                    groups.push((count_a + groups[7].0, count_b + groups[7].1));
                },
                _ => {
                    groups.push((count_a, count_b));
                },
            }
            i += 1;
        }
        groups
    }

    fn get_temperament(&self) -> String {
        let groups = self.tally();
        let mut temperment = String::new();
        for (i, group) in groups.iter().enumerate() {
            if i == 0 {
                if group.0 > group.1 {
                    temperment.push('E');
                } else if group.0 < group.1 {
                    temperment.push('I');
                } else {
                    temperment.push('X');
                }
            } else if i == 3 {
                if group.0 > group.1 {
                    temperment.push('S');
                } else if group.0 < group.1 {
                    temperment.push('N');
                } else {
                    temperment.push('X');
                }
            } else if i == 6 {
                if group.0 > group.1 {
                    temperment.push('T');
                }  else if group.0 < group.1 {
                    temperment.push('F');
                } else {
                    temperment.push('X');
                }
            } else if i == 9 {
                if group.0 > group.1 {
                    temperment.push('J');
                } else if group.0 < group.1 {
                    temperment.push('P');
                } else {
                    temperment.push('X');
                }
            }
        }
        temperment
    }
}


#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum Options {
    A(String),
    B(String),
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
enum Answer {
    A,
    B,
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
struct Temperaments {
    artisan: String,
    guardian: String,
    idealist: String,
    rational: String,
}
#[derive(Debug, Deserialize, PartialEq)]
enum TemperamentType {
    Artisan,
    Guardian,
    Idealist,
    Rational,
}

impl Display for TemperamentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemperamentType::Artisan => write!(f, "- Artisan: \n"),
            TemperamentType::Guardian => write!(f, "- Guardian: \n"),
            TemperamentType::Idealist => write!(f, "- Idealist: \n"),
            TemperamentType::Rational => write!(f, "- Rational: \n"),
        }
    }

}
impl Temperaments {
    fn print(&self, temperment: TemperamentType) {
        match temperment {
            TemperamentType::Artisan => println!("{}\n{}", temperment, self.artisan),
            TemperamentType::Guardian => println!("{}\n{}", temperment, self.guardian),
            TemperamentType::Idealist => println!("{}\n{}", temperment, self.idealist),
            TemperamentType::Rational => println!("{}\n{}", temperment, self.rational),
        }
    }

}

fn set_color(color: Color) {
    execute!(std::io::stdout(), SetForegroundColor(color)).unwrap();
}

fn clear_terminal() {
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_score() {
        let score = Score::new(Answer::A);
        assert_eq!(score.value, Answer::A);
    }

    #[test]
    fn test_scoring_grid() {
        let mut scoring_grid = ScoringGrid::new();
        scoring_grid.add_score(Score::new(Answer::A));
        scoring_grid.add_score(Score::new(Answer::B));
        scoring_grid.add_score(Score::new(Answer::B));
        scoring_grid.add_score(Score::new(Answer::A));
        scoring_grid.add_score(Score::new(Answer::A));
        scoring_grid.add_score(Score::new(Answer::A));
        scoring_grid.add_score(Score::new(Answer::A));
        scoring_grid.add_score(Score::new(Answer::B));

        // On the the 8th addition it will add a new array to the scores array
        assert_eq!(scoring_grid.scores.len(), 2);

    }

    #[test]
    fn test_talley() {
        let mut keirsey: Keirsey = Keirsey::new(
            serde_json::from_reader(
                BufReader::new(
                    File::open("questions.json").unwrap()
                )
            ).unwrap()
        );

        for _ in keirsey.questionnaire.questions.iter().enumerate() {
            keirsey.answer_grid.add_score(Score::new(Answer::A));
        }

        keirsey.answer_grid.tally();

        // The tally method should return a vector of tuples with the first value being the number of A's and the second being the number of B's
        assert_eq!(keirsey.answer_grid.tally(), vec![(10, 0), (10, 0), (10, 0), (20, 0), (10, 0), (10, 0), (20, 0), (10, 0), (10, 0), (20, 0)]);
    }

    #[test]
    fn test_temperaments() {
        let mut keirseys:Vec<Keirsey> = vec![];

        let og_keirsey = Keirsey::new(
            serde_json::from_reader(
                BufReader::new(
                    File::open("questions.json").unwrap()
                )
            ).unwrap()
        );

        for _ in 0..16 {
            keirseys.push(og_keirsey.clone())
        }

        for (i, mut keirsey) in keirseys.iter_mut().enumerate() {
            for j in 0..70 {
                let pattern_variable = match i {
                    0 => false,                                                                             // ESTJ
                    1 => j % 7 == 0,                                                                        // ISTJ
                    2 => (j-1) % 7 == 0 || j == 2,                                                          // ENTJ
                    3 => (j-2) % 7 == 0 && j != 2 || (j-3) % 7 == 0 || j == 4,                              // ESFJ
                    4 => (j-3) % 7 == 0 && j == 4 || (j-5) % 7 == 0 || j == 6,                              // ESTP
                    5 => j % 7 == 0 || (j-1) % 7 == 0 || j == 2,                                            // INTJ
                    6 => j % 7 == 0 || (j-2) % 7 == 0 && j != 2 || (j-3) % 7 == 0 || j == 4,                // ISFJ
                    7 => j % 7 == 0 || (j-5) % 7 == 0 || j == 6,                                            // ISTP
                    8 => (j-1) % 7 == 0 || j == 2 || (j-3) % 7 == 0 || j == 4,                              // ENFJ
                    9 => (j-3) % 7 == 0 || j == 4 || (j-5) % 7 == 0 || j == 6,                              // ESFP
                    10 => (j-1) % 7 == 0 || j == 2 || (j-5) % 7 == 0 || j == 6,                             // ENTP
                    11 => j % 7 == 0 || (j-1) % 7 == 0 || j == 2 || (j-3) % 7 == 0 || j == 4,               // INFJ
                    12 => j % 7 == 0 || (j-3) % 7 == 0 || j == 4 || (j-5) % 7 == 0 || j == 6,               // ISFP
                    13 => j % 7 == 0 || (j-1) % 7 == 0 || j == 2 || (j-5) % 7 == 0 || j == 6,               // INTP
                    14 => (j-1) % 7 == 0 || j == 2 || (j-3) % 7 == 0 || j == 4 || (j-5) % 7 == 0 || j == 6, // ENFP
                    _ => {true}                                                                             // INFP
                };
                let score = if pattern_variable {
                    Score::new(Answer::B)
                } else {
                    Score::new(Answer::A)
                };
                keirsey.answer_grid.add_score(score);
            }
        }

        let answer_array = ["ESTJ","ISTJ","ENTJ","ESFJ","ESTP","INTJ","ISFJ","ISTP","ENFJ","ESFP","ENTP","INFJ","ISFP","INTP","ENFP","INFP"];

        for (i, keirsey) in keirseys.iter().enumerate() {
            assert_eq!(keirsey.answer_grid.get_temperament(), answer_array[i], "{:?}", keirsey.answer_grid);
        }

    }

    #[test]
    fn test_temperament_type() {
        let mut possible_temperaments
            = vec![];

        let possible = [["E", "I", "X"], ["S", "N", "X"],["F", "T", "X"], ["J", "P", "X"]];


        for &first_letter in possible[0].iter() {
            for &second_letter in possible[1].iter() {
                for &third_letter in possible[2].iter() {
                    for &fourth_letter in possible[3].iter() {
                        let combination = format!("{}{}{}{}", first_letter, second_letter, third_letter, fourth_letter);
                        possible_temperaments.push(combination);
                    }
                }
            }
        }

        for temperament in possible_temperaments {
            // How do i test this?
        }

    }

    #[test]
    fn call_teh_avatar() {
        let mut keirsey = Keirsey::new(
            serde_json::from_reader(
                BufReader::new(
                    File::open("questions.json").unwrap()
                )
            ).unwrap()
        );
        for j in 0..70 {
            if j % 2 == 0 {
                keirsey.answer_grid.add_score(Score::new(Answer::B));
            } else {
                keirsey.answer_grid.add_score(Score::new(Answer::A));
            }
        }
        let temp = keirsey.answer_grid.get_temperament();
        assert_eq!(temp, "XXXX");
        assert_eq!(keirsey.print_temperament(), welcome_the_avatar())
    }
}