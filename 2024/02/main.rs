#[path = "../../libs/helper.rs"]
mod helper;

fn safety_check(input: &Vec<i32>) -> bool {
    let mut updown: &str = "none";

    for i in 0..input.len() - 1 {
        if updown == "none" {
            if input[i] < input[i + 1] {
                updown = "up";
            } else if input[i] > input[i + 1] {
                updown = "down";
            } else {
                return false;
            }
        }

        if updown == "up" && input[i] >= input[i + 1] {
            return false;
        }

        if updown == "down" && input[i] <= input[i + 1] {
            return false;
        }
        if (input[i] - input[i + 1]).abs() > 3 {
            return false;
        }
    }
    return true;
}

fn main() {
    let lines = helper::lines_from_file("./input.txt");

    let mut fullinput: Vec<Vec<i32>> = vec![];

    for line in lines {
        let mut tmp: Vec<i32> = vec![];

        let str_input: Vec<&str> = line.trim().split(' ').collect();
        for i in 0..str_input.len() {
            tmp.push(str_input[i].parse::<i32>().expect("unable to parse"));
        }
        fullinput.push(tmp);
    }

    // part 1 - safe reports : 483

    let mut safe_reports = 0;

    for input in &fullinput {
        if safety_check(input) {
            safe_reports += 1;
        }
    }

    println!("safe reports: {}", safe_reports);

    // part 2 - engage dampeners
    println!("engaging dampeners");

    let mut dampened_reports = 0;
    for input in fullinput {
        if safety_check(&input) {
            dampened_reports += 1;
        } else {
            for i in 0..input.len() {

                let mut tmp_input: Vec<i32> = vec![];

                for n in 0..input.len() {
                    if  n != i {
                        tmp_input.push(input[n]);
                    }
                }
                if safety_check(&tmp_input) {
                    dampened_reports += 1;
                    break;
                }
            }
        }
    }

    println!("dampened reports: {}", dampened_reports);
}
