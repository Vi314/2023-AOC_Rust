use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let cfg = CubeSettings {
        red_max: 12,
        green_max: 13,
        blue_max: 14,
    };
    let draw_rgx: Regex = Regex::new(r"(\d{1,2}) (green|red|blue)").unwrap();
    let id_rgx: Regex = Regex::new(r"(\d{1,3}):").unwrap();
    let mut id_total: i32 = 0; //Result Variable

    let input_string = read_to_string("src\\input.txt").unwrap();
    let games: Vec<&str>= input_string.split("\n").collect();
    for game in games {
        let draws: Vec<&str> = draw_rgx.find_iter(game).map(|x| x.as_str()).collect();
        let game_id:i32 = id_rgx.find(game).unwrap().as_str().replace(":", "").parse().unwrap();
        
        let mut is_game_ok = true;
        for draw in draws {
            let split: Vec<&str> = draw.split(" ").collect();
            let value:i32 = split[0].parse().unwrap();
            let color = split[1];
            match color {
                "red" => if value > cfg.red_max{
                    println!("RED HIT");
                    is_game_ok = false;
                    break;
                },
                "green" => if value > cfg.green_max{
                    println!("GREEN HIT");
                    is_game_ok = false;
                    break;
                },
                "blue" => if value > cfg.blue_max{
                    println!("BLUE HIT");
                    is_game_ok = false;
                    break;
                },
                _ => continue
            }
        }
        if is_game_ok {
            id_total = id_total + game_id;
        }
    }
    println!("RESULT ID TOTAL IS => {}", id_total); // SEEMS TO JUST CALCULATE UP TO 100 ~ no exclusions
}

struct CubeSettings {
    red_max: i32,
    green_max: i32,
    blue_max: i32,
}
