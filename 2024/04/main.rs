#[path = "../../libs/helper.rs"]
mod helper;

const MAX_SIZE: isize = 140;

fn get_char_on_relative_position(input: &Vec<Vec<char>>, pos_x: isize, pos_y: isize) -> char {
    if pos_x < 0 || pos_x >= MAX_SIZE || pos_y < 0 || pos_y >= MAX_SIZE {
        return '.';
    }
    return input[pos_x as usize][pos_y as usize];
}

fn get_tile_in_direction(
    input: &Vec<Vec<char>>,
    pos_x: isize,
    pos_y: isize,
    direction: isize,
    dist: isize,
) -> char {
    // direction is mathematical counter-clockwise
    // from .->0 over .->1 to .->7 (8 directions)
    //  321             y
    //  4.0             ^
    //  567   asuming:  . -> x

    if direction == 0 {
        return get_char_on_relative_position(&input, pos_x + dist, pos_y);
    }
    if direction == 1 {
        return get_char_on_relative_position(&input, pos_x + dist, pos_y + dist);
    }
    if direction == 2 {
        return get_char_on_relative_position(&input, pos_x, pos_y + dist);
    }
    if direction == 3 {
        return get_char_on_relative_position(&input, pos_x - dist, pos_y + dist);
    }
    if direction == 4 {
        return get_char_on_relative_position(&input, pos_x - dist, pos_y);
    }
    if direction == 5 {
        return get_char_on_relative_position(&input, pos_x - dist, pos_y - dist);
    }
    if direction == 6 {
        return get_char_on_relative_position(&input, pos_x, pos_y - dist);
    }
    if direction == 7 {
        return get_char_on_relative_position(&input, pos_x + dist, pos_y - dist);
    }

    return '.';
}

fn main() {
    // part 1 - finding xmas
    println!("finding XMAS, lets go!");

    let input = helper::get_2d_vector_from_file("./input.txt");

    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut xmas_counter = 0;
    println!("input size is: {},{}", input[0].len(), input.len());

    for x in 0..MAX_SIZE {
        for y in 0..MAX_SIZE {
            /*
             * 3..3..3
             * .2.2.2.
             * ..111..
             * 3210123
             * ..111..
             * .2.2.2.
             * 3..3..3
             *
             */

            if xmas[0] != get_char_on_relative_position(&input, x, y) {
                continue;
            }

            for dir in 0..8 {
                let mut found = true;
                for dist in 1..4 {
                    if xmas[dist] == get_tile_in_direction(&input, x, y, dir, dist as isize) {
                        continue;
                    }
                    found = false;
                }
                if found == true {
                    xmas_counter += 1;
                }
            }
        }
    }

    println!("found {} XMAS'es", xmas_counter);

    // part 2 -- oh, noes! it' an X-MAS!
    let mut x_dash_mas_count = 0;

    for x in 1..MAX_SIZE - 1 {
        for y in 1..MAX_SIZE - 1 {
            /*
             * "only" 4 variations are possible.
             * all centered around 'A' in the middle.
             * M.S M.S S.M S.M
             * .A. .A. .A. .A.
             * M.S S.M M.S S.M
             */
            if 'A' != get_char_on_relative_position(&input, x, y) {
                continue;
            }

            // only use dirs: 1', '3, .5, 7.
            if ('M' == get_tile_in_direction(&input, x, y, 3, 1)
                && 'S' == get_tile_in_direction(&input, x, y, 7, 1))
                || ('M' == get_tile_in_direction(&input, x, y, 7, 1)
                    && 'S' == get_tile_in_direction(&input, x, y, 3, 1))
            {
                if ('M' == get_tile_in_direction(&input, x, y, 1, 1)
                    && 'S' == get_tile_in_direction(&input, x, y, 5, 1))
                    || ('M' == get_tile_in_direction(&input, x, y, 5, 1)
                        && 'S' == get_tile_in_direction(&input, x, y, 1, 1))
                {
                    x_dash_mas_count += 1;
                }
            }
        }
    }

    println!("cound {} X-MAS'es", x_dash_mas_count);
}
