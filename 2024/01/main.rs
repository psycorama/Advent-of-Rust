#[path = "../../libs/helper.rs"] mod helper;

fn main() {
    let lines = helper::lines_from_file("./input.txt");

    let mut first_vec: Vec<i32> = vec![];
    let mut second_vec: Vec<i32> = vec![];
    for line in lines {
        let input: Vec<&str> = line.trim().split("   ").collect();
        first_vec.push(input[0].parse::<i32>().expect("unable to parse"));
        second_vec.push(input[1].parse::<i32>().expect("unable to parse"));
    }

    first_vec.sort();
    second_vec.sort();

    // first part - distance

    let mut sum_of_dist: i32 = 0;

    for i in 0..first_vec.len() {
        let dist = first_vec[i] - second_vec[i];
        sum_of_dist += dist.abs();
    }

    println!("first part - Sum of distance is: {}", sum_of_dist);

    // second part - similarities

    let mut sum_of_similarities = 0;

    for i in first_vec.iter_mut() {
        for n in second_vec.iter_mut() {
            if *i == *n {
                sum_of_similarities += *n;
            }
        }
    }
    println!("second part - Similarity is: {}", sum_of_similarities);
}
