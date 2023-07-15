// "cargo run"
fn main() {
    println!("-*- -*- -*- -*- -*- -*- -*-");
    practice_option();
    println!("-*- -*- -*- -*- -*- -*- -*-");
    practice_loop();
}

fn practice_loop() {
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

    fn car_factory(order: i32, miles: u32) -> Car {
        let colors = ["Blue", "Green", "Red", "Silver"];

        let mut color = order as usize;
        while color > 4 {
            color = color - 4;
        }

        let mut motor = Transmission::Manual;
        let mut roof = true;
        if order % 3 == 0 {
            motor = Transmission::Automatic;
        } else if order % 2 == 0 {
            motor = Transmission::SemiAuto;
            roof = false;
        }

        Car {
            color: String::from(colors[(color - 1) as usize]),
            motor: motor,
            roof: roof,
            age: car_quality(miles),
        }
    }

    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();

    let mut car: Car;

    let mut miles = 0;

    for order in 1..12 {
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }
}

fn practice_option() {
    struct Person {
        first: String,
        middle: Option<String>,
        last: String,
    }

    fn build_full_name(person: &Person) -> String {
        let mut full_name = String::new();
        full_name.push_str(&person.first);
        full_name.push_str(" ");

        if let Some(middle) = &person.middle {
            full_name.push_str(&middle);
            full_name.push_str(" ")
        }

        full_name.push_str(&person.last);
        full_name
    }

    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}
