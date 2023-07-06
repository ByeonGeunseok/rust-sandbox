// "cargo run"
fn main() {
    // Basic print macro
    println!("Hello, world!");

    // Print with arguments
    println!("Arg1:{}, arg2:{}, arg3:{}, arg4:{}", "ONE", "two", 'C', 4);

    // This Todo! macro will return panic message.
    todo!("This is TODO panic message.");
}
