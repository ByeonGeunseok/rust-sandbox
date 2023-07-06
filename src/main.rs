// "cargo run"
fn main() {
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
