use regex::Regex;
use std::fs::read_to_string;

fn main() {
    println!("TASK ONE");
    task_one();
    println!("****************************************************");
    println!("TASK TWO");
    task_two();
    println!("****************************************************");
}

fn task_two() {
    let draw_rgx: Regex = Regex::new(r"(\d{1,2}) (green|red|blue)").unwrap();
    let mut id_total: i32 = 0; //Result Variable

    let input_string = read_to_string("src\\input.txt").unwrap();
    //The games are separated by the line break character
    let games: Vec<&str> = input_string.split("\n").collect();
    for game in games {
        // We find the values in a given game that match the draw regex
        // The format is => "-number- -color-" e.g "24 red"
        // We put those draws as borrowed string slices in a vector
        let draws: Vec<&str> = draw_rgx.find_iter(game).map(|x| x.as_str()).collect();

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for draw in draws {
            // We get the number and color from a string like "9 red"
            let split: Vec<&str> = draw.split(" ").collect();
            let value: i32 = split[0].parse().unwrap();
            let color = split[1];
            // We check if the amount drawn for the given color is allowed as per the "cfg" rules
            // In the case the amount drawn exceeds amount allowed we mark "is_draw_allowed" as false and do not add it to "id_total"
            match color {
                "red" => {
                    if value > max_red {
                        max_red = value;
                    }
                }
                "green" => {
                    if value > max_green {
                        max_green = value;
                    }
                }
                "blue" => {
                    if value > max_blue {
                        max_blue = value;
                    }
                }
                _ => continue,
            }
        }
        id_total = id_total + (max_blue * max_green * max_red);
    }
    println!("RESULT ID TOTAL IS => {}", id_total);
}

fn task_one() {
    let cfg = CubeSettings {
        red_max: 12,
        green_max: 13,
        blue_max: 14,
    };
    let draw_rgx: Regex = Regex::new(r"(\d{1,2}) (green|red|blue)").unwrap();
    let id_rgx: Regex = Regex::new(r"(\d{1,3}):").unwrap();
    let mut id_total: i32 = 0; //Result Variable

    let input_string = read_to_string("src\\input.txt").unwrap();
    //The games are separated by the line break character
    let games: Vec<&str> = input_string.split("\n").collect();
    for game in games {
        // We find the values in a given game that match the draw regex
        // The format is => "-number- -color-" e.g "24 red"
        // We put those draws as borrowed string slices in a vector
        let draws: Vec<&str> = draw_rgx.find_iter(game).map(|x| x.as_str()).collect();
        // We get the game id from a string like "Game 1:" with regex and some string functions
        let game_id: i32 = id_rgx
            .find(game)
            .unwrap()
            .as_str()
            .replace(":", "")
            .parse()
            .unwrap();

        let mut is_draw_allowed = true;
        for draw in draws {
            // We get the number and color from a string like "9 red"
            let split: Vec<&str> = draw.split(" ").collect();
            let value: i32 = split[0].parse().unwrap();
            let color = split[1];
            // We check if the amount drawn for the given color is allowed as per the "cfg" rules
            // In the case the amount drawn exceeds amount allowed we mark "is_draw_allowed" as false and do not add it to "id_total"
            match color {
                "red" => {
                    if value > cfg.red_max {
                        is_draw_allowed = false;
                        break;
                    }
                }
                "green" => {
                    if value > cfg.green_max {
                        is_draw_allowed = false;
                        break;
                    }
                }
                "blue" => {
                    if value > cfg.blue_max {
                        is_draw_allowed = false;
                        break;
                    }
                }
                _ => continue,
            }
        }
        if is_draw_allowed {
            id_total = id_total + game_id;
        }
    }
    println!("RESULT ID TOTAL IS => {}", id_total);
}

struct CubeSettings {
    red_max: i32,
    green_max: i32,
    blue_max: i32,
}
