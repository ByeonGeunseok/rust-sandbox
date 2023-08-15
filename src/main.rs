// "cargo run"
fn main() {
    repeat_numbers();
}

fn repeat_numbers() {
    use rand::Rng;
    use std::collections::HashMap;
    use std::io::{self, Write};
    use std::time::Instant;

    let mut result_list = HashMap::new();

    print!("How many numbers do you need? (Press '!q' to quit.):");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut amount_input = String::new();
    io::stdin()
        .read_line(&mut amount_input)
        .expect("Failed to read line");
    let amount: i32 = amount_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    print!("Choose a minimum range. (Press '!q' to quit.):");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut min_input = String::new();
    io::stdin()
        .read_line(&mut min_input)
        .expect("Failed to read line");
    let min: i32 = min_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    print!("Choose a maximum range. (Press '!q' to quit.):");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut max_input = String::new();
    io::stdin()
        .read_line(&mut max_input)
        .expect("Failed to read line");
    let max: i32 = max_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    print!("How many times repeat? (Press '!q' to quit.):");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut repeat_input = String::new();
    io::stdin()
        .read_line(&mut repeat_input)
        .expect("Failed to read line");
    let repeat: i32 = repeat_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let mut _loop_cnt = 0;
    let mut _result_cnt = 0;

    let start_time = Instant::now();

    loop {
        let i = rand::thread_rng().gen_range(min..max + 1);
        let count = result_list.entry(i).or_insert(0);
        *count += 1;
        _loop_cnt += 1;

        if _loop_cnt >= repeat {
            break;
        }
    }
    let mut sorted_pairs: Vec<(&i32, &i32)> = result_list.iter().collect();
    sorted_pairs.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

    for (key, value) in &sorted_pairs {
        println!("Key: {}, Value: {}", key, value);
        _result_cnt += 1;
        if _result_cnt >= amount {
            break;
        }
    }

    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Elapsed time: {:?}", elapsed_time);
}
