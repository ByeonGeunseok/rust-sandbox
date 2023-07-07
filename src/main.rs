// "cargo run"
fn main() {
    println!("-*- -*- -*- -*- -*- -*- -*-");
    practice_array();
    println!("-*- -*- -*- -*- -*- -*- -*-");
    practice_vector();
}

fn practice_array() {
    let arr_week = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let arr_init = [0; 7];

    let monday = arr_week[1];
    let thursday = arr_week[4];

    println!("{:?}", arr_init);
    println!("SUN,{},TUE,WED,{},FRI,SAT", monday, thursday);
}

fn practice_vector() {
    let three_nums = vec![1, 2, 3];
    let zeros = vec![0; 5];

    println!("{:?}", three_nums);
    println!("{:?}", zeros);

    let mut alphabets = Vec::new();

    alphabets.push("A");
    alphabets.push("B");
    println!("{:?}", alphabets);

    alphabets.push("D");
    println!("{:?}", alphabets);

    println!("{:?}", alphabets.pop());

    alphabets.push("C");
    println!("{:?}", alphabets);

    alphabets[2] = "c";
    println!("{:?}", alphabets);

    let beyond = alphabets[4];
    println!("{}", beyond);
}
