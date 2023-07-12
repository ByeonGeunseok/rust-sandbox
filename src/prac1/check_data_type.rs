fn check_data_type() {
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
}

