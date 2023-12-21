use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let number_code_rgx = Regex::new(r"(\d{1,3})").unwrap();
    let mut gear_code_total: i32 = 0;
    let input_string = read_to_string(Path::new("src/input.txt")).unwrap().replace("\r", "").replace("\n", "");
    let line_length = 140;//input_string.chars().take_while(|x| x != &'\r' && x != &'\n').count();
    let input_chars: Vec<char> = input_string.chars().collect();
    let gear_codes: Vec<GearCode> = 
        number_code_rgx
        .find_iter(&input_string)
        .map(|cap| 
            GearCode{
                left: cap.start()
                , right: cap.end()
                , val: cap.as_str().parse().unwrap_or(0)})
        .collect();
    println!("{}", input_string);
    for gc in gear_codes {
        let left_pos = gc.left - 1;
        let right_pos = gc.right + 1;

        let left_char = input_chars.get(left_pos).unwrap_or(&'.');
        let right_char = input_chars.get(right_pos).unwrap_or(&'.');
        if left_char != &'.' || right_char != &'.' {
            gear_code_total += gc.val;
            continue;
        }
        if (left_pos) as i32 - (line_length) as i32 > 0 {
            let top_chars = &input_chars[(left_pos - line_length)..(right_pos - line_length)];
            // println!("TOP CHARS => {:#?}", top_chars);
            if top_chars.iter().any(|x| x != &'.') {
                gear_code_total += gc.val;
                continue;
            }
        }
        if right_pos + line_length < input_chars.len() {
            // println!("R_POS {} VAL {} R_POS_VAL  {}", right_pos, gc.val, input_chars[right_pos]);
            let bottom_chars = &input_chars[(left_pos + line_length)..(right_pos + line_length)];
            // println!("BOTTOM CHARS => {:#?}", bottom_chars);
            if bottom_chars.iter().any(|x| x != &'.') {
                gear_code_total += gc.val;
                continue;
            }
        }
    }
    println!("GEAR CODE TOTAL => {}", gear_code_total);
}

struct GearCode {
    left: usize,
    right: usize,
    val: i32
}
