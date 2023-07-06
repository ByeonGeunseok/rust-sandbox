// "cargo run"
fn main() {
    let int_number: u32 = 11;
    let float_number: f32 = 5.4;
    let is_bigger = 1 > 4;

    println!("int: {}, float: {}", int_number, float_number);

    println!("Is 1 bigger than 4? : {}", is_bigger);

    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    let character_1: char = 'S';
    let character_2: char = 'f';
    let smiley_face = '😀';
    let string_1 = "miley ";
    let string_2: &str = "ace";

    println!(
        "{} is a {}{}{}{}",
        smiley_face, character_1, string_1, character_2, string_2
    );

    check_var();
    check_shadowing();
    check_print();
}

fn check_print() {
    // Basic print macro
    println!("Hello, world!");

    // Print with arguments
    println!("Arg1:{}, arg2:{}, arg3:{}, arg4:{}", "ONE", "two", 'C', 4);

    // This Todo! macro will return panic message.
    todo!("This is TODO panic message.");
}

fn check_var() {
    let a_number = 10;
    let a_word = "TEN";
    let mut re_number = 11;
    let mut re_word = "ELEVEN";

    println!("The number: {}", a_number);
    println!("The word: {}", a_word);

    println!("Check a mutable value: {}, {}", re_number, re_word);
    re_number = 9;
    re_word = "NINE";
    println!("Check a mutable value: {}, {}", re_number, re_word);
}

fn check_shadowing() {
    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;

    println!("The shadow number is {}.", shadow_num);
}
