// "cargo run"
fn main() {
    println!("-*- -*- -*- -*- -*- -*- -*-");
    practice_if_else();
    println!("-*- -*- -*- -*- -*- -*- -*-");
    practice_array();
    practice_vector();
    practice_more_types();
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

fn practice_more_types() {
    #[derive(PartialEq, Debug)]
    struct Car {
        color: String,
        motor: Transmission,
        roof: bool,
        age: (Age, u32),
    }

    #[derive(PartialEq, Debug)]
    enum Transmission {
        Manual,
        SemiAuto,
        Automatic,
    }

    #[derive(PartialEq, Debug)]
    enum Age {
        New,
        Used,
    }

    fn car_quality(miles: u32) -> (Age, u32) {
        let quality: (Age, u32) = (Age::New, miles);

        quality
    }

    fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
        Car {
            color: color,
            motor: motor,
            roof: roof,
            age: car_quality(miles),
        }
    }

    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut car: Car;
    let mut engine = Transmission::Manual;

    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
}

fn practice_if_else() {
    let is_t = true;
    let greeting = if is_t { "RUST" } else { "rust" };

    println!("{}", greeting);

    #[derive(PartialEq, Debug)]
    struct Car {
        color: String,
        motor: Transmission,
        roof: bool,
        age: (Age, u32),
    }

    #[derive(PartialEq, Debug)]
    enum Transmission {
        Manual,
        SemiAuto,
        Automatic,
    }

    #[derive(PartialEq, Debug)]
    enum Age {
        New,
        Used,
    }

    fn car_quality(miles: u32) -> (Age, u32) {
        if miles > 0 {
            return (Age::Used, miles);
        }
        (Age::New, miles)
    }

    fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
        if car_quality(miles).0 == Age::Used {
            if roof {
                println!(
                    "Preparing a used car: {:?}, {}, Hard top, {} miles",
                    motor, color, miles
                );
            } else {
                println!(
                    "Preparing a used car: {:?}, {}, Convertible, {} miles",
                    motor, color, miles
                );
            }
        } else {
            if roof {
                println!(
                    "Building a new car: {:?}, {}, Hard top, {} miles",
                    motor, color, miles
                );
            } else {
                println!(
                    "Building a new car: {:?}, {}, Convertible, {} miles",
                    motor, color, miles
                );
            }
        }

        Car {
            color,
            motor,
            roof,
            age: car_quality(miles),
        }
    }

    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
