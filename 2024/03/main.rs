#[path = "../../libs/helper.rs"]
mod helper;

extern crate regex;
use regex::Regex;

fn get_muls_from_string(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut mul_result = 0;

    for results in re.captures_iter(&input) {
        let first = &results[1].parse::<i32>().expect("unable to parse");
        let second = &results[2].parse::<i32>().expect("unable to parse");

        mul_result += first * second;
    }

    return mul_result;
}

fn get_muls_with_do_dont(input: &str) -> i32 {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don\'t\(\))").unwrap();
    let mut mul_result = 0;
    let mut do_mul = true;

    for results in re.captures_iter(&input) {
        if &results[0] == "do()" {
            do_mul = true;
            continue;
        } else if &results[0] == "don't()" {
            do_mul = false;
            continue;
        } else if do_mul == true {
            mul_result += get_muls_from_string(&results[1]);
        }
    }

    return mul_result;
}

fn main() {
    // read-in
    let lines = helper::lines_from_file("./input.txt");

    let mut full_input: String = "".to_string();

    for i in 0..lines.len() {
        full_input.push_str(lines[i].as_str());
    }

    // first part - multiply
    let mul_result = get_muls_from_string(full_input.as_str());

    println!("Mulling it over. sum is: {}", mul_result);

    // second part - do() or don't(), there is no try

    let do_mul_res = get_muls_with_do_dont(full_input.as_str());

    println!("Do()ing it over. sum is: {}", do_mul_res);
}
