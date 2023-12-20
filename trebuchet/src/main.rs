use std::fs;

fn main() {
    trebuchet();
}

fn trebuchet() {
    let path = r#"C:\Users\Abdullah\Desktop\Trebuchet.txt"#;
    let file_string = fs::read_to_string(path).unwrap();
    let string_but_different = file_string;
    let string_vec: Vec<&str> = string_but_different.split("\n").collect();
    for i in &string_vec {
        println!("{}", i);
    }
    let mut counter: i32 = 0;
    for val in string_vec {
        let val2 = val                  //There be some wild cases where shit like eighthree combines and one of em gets shot
            .replace("one", "on1e")     //so you know what homeboy did ?
            .replace("two", "t2wo")     //fuckin went and looked what was possible and placed the numbers
            .replace("three", "t3hree") //strategically so them conflicts couldnt happen
            .replace("four", "four4")
            .replace("five", "5five")
            .replace("six", "6six")
            .replace("seven", "seven7")
            .replace("eight", "8eight")
            .replace("nine", "9nine");

        let numbers: Vec<char> = val2
            .chars()
            .filter(|x| x.is_numeric())
            .into_iter()
            .collect();
        let char_one = numbers.iter().next().unwrap();
        let char_two = numbers.iter().next_back().unwrap();
        let nums_str: String = vec![char_one, char_two].into_iter().collect();
        // print!("{} === ", nums_str);
        let num: i32 = nums_str.parse().unwrap();
        // println!("{}", num);
        counter += num;
    }
    println!("Result {}", counter);
}
