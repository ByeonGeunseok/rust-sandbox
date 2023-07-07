// "cargo run"
fn main() {
    println!("==========");
    println!("==========");
    println!("==========");
    println!("==========");
    check_var();
    check_shadowing();
    check_data_type();
    check_structure();
    check_enum_variants();
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

fn check_structure() {
    struct Sample(char, i32, bool);
    struct Student {
        name: String,
        level: u8,
        remote: bool,
    } // semi-colon is unnecessary.
    struct Grades(char, char, char, char, f32);

    let tuple_e = Sample('E', 4i32, true);

    let user_1 = Student {
        name: String::from("John Doe"),
        level: 5,
        remote: false,
    };
    let user_2 = Student {
        level: 3,
        remote: true,
        name: String::from("Jane Doe"),
    };

    let mark_1 = Grades('A', 'B', 'B', 'C', 3.0);
    let mark_2 = Grades('C', 'A', 'A', 'A', 3.5);

    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );
    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    );

    println!(
        "first: {}, second: {}, third: {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );
}

fn check_enum_variants() {
    #[derive(Debug)]
    struct KeyPress(String, char);

    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }

    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress),
    }

        let click = MouseClick { x: 100, y: 250 };
        println!("Mouse click location: {}, {}", click.x, click.y);

        let keys = KeyPress(String::from("Ctrl +"), 'N');
        println!("\nKeys pressed: {}{}", keys.0, keys.1);

        let we_load = WebEvent::WELoad(true);
        let we_click = WebEvent::WEClick(click);
        let we_key = WebEvent::WEKeys(keys);

        println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
}
