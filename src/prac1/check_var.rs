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

