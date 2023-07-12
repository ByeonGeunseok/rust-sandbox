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

