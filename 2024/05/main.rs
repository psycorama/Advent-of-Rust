#[path = "../../libs/helper.rs"]
mod helper;

fn parse_input(input: Vec<String>) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut ordering_rules: Vec<(i32, i32)> = vec![];
    let mut page_numbers: Vec<Vec<i32>> = vec![];

    let mut part = 1;

    for line in input {
        if line.is_empty() {
            part = 2;
            continue;
        }
        if part == 1 {
            let values: Vec<&str> = line.trim().split("|").collect();
            ordering_rules.push((
                values[0].parse::<i32>().expect("unable to parse"),
                values[1].parse::<i32>().expect("unable to parse"),
            ));
        } else {
            let values: Vec<&str> = line.trim().split(",").collect();
            let mut num_val: Vec<i32> = vec![];
            for i in 0..values.len() {
                num_val.push(values[i].parse::<i32>().expect("unable to parse"));
            }
            page_numbers.push(num_val);
        }
    }

    return (ordering_rules, page_numbers);
}

fn check_first(ordering_rules: &Vec<(i32, i32)>, first: i32) -> bool {
    for &(_, i) in ordering_rules {
        if i == first {
            return false;
        }
    }
    return true;
}

fn check_ordering(ordering_rules: &Vec<(i32, i32)>, first: i32, second: i32) -> bool {
    for rule in ordering_rules {
        let (a, b) = rule;

        // if a == &first && b == &second {
        //     return true;
        // }
        if b == &first && a == &second {
            return false;
        }
    }
    return true;
}

fn get_middle_entry(page: &Vec<i32>) -> i32 {
    //println!(" adding {}", page[page.len() / 2]);
    return page[page.len() / 2];
}

fn main() {
    // day 5 - part 1

    let raw_input = helper::lines_from_file("./input.txt");
    let (ordering_rules, page_numbers) = parse_input(raw_input);

    // check read-in
    /*
    for i in 0..10 {
        let (a, b) = ordering_rules[i];
        println!("ordering rule: {}->{}", a, b);
    }

    for i in 0..10 {
        print!("page numbers: ");
        for n in &page_numbers[i] {
            print!("{}, ", n);
        }
        println!();
    }
    */

    let mut incorrect_page_inputs: Vec<Vec<i32>> = vec![];
    let mut correct_page_sum = 0;
    for pages in page_numbers {
        let mut valid_page = true;

        for n in 0..(pages.len() - 1) {
            for i in (n + 1)..(pages.len()) {
                if !check_ordering(&ordering_rules, pages[n], pages[i]) {
                    valid_page = false;
                    incorrect_page_inputs.push(pages.clone());
                    break;
                }
            }
            if !valid_page {
                break;
            }
        }
        if valid_page {
            correct_page_sum += get_middle_entry(&pages);
        }
    }
    println!("found corrected page sum: {}", correct_page_sum);

    // part 2 - use only incorrect pages, order them by known rules, add middle page
    // example sum is 123

    let mut incorrect_page_sum = 0;

    for mut pages in incorrect_page_inputs {
        let mut valid_page = true;
        for n in 0..(pages.len() -1) {
            for i in ( (n + 1)..(pages.len())) {
                if !check_ordering(&ordering_rules, pages[n], pages[i]) {
                    &pages.swap(n, i);
                    if !check_ordering(&ordering_rules, pages[n], pages[i]) {
                        valid_page = false;
                        break;
                    }
                }
            }
            if !valid_page{
                break;
            }
        }
        if valid_page {
            incorrect_page_sum += get_middle_entry(&pages);
        }
    }
    println!("found incorrected page sum: {}", incorrect_page_sum);
}
