// "cargo run"
fn main() {
    practice_array();
}

fn practice_array() {
    let arr_week = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let arr_init = [0; 7];

    let monday = arr_week[1];
    let thursday = arr_week[4];

    println!("{:?}", arr_init);
    println!("SUN,{},TUE,WED,{},FRI,SAT", monday, thursday);
}
