fn check_shadowing() {
    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;

    println!("The shadow number is {}.", shadow_num);
}

