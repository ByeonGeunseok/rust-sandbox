// "cargo run"
fn main() {
    println!("-*- -*- -*- -*- -*- -*- -*-");
    println!("-*- -*- -*- -*- -*- -*- -*-");
    practice_loop();
    practice_option();
    practice_result();
    practice_lifetime();
    practice_generic();
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

fn practice_result() {
    use std::fs::File;
    use std::io::{Error, Read};
    use std::path::PathBuf;

    fn read_file_contents(path: PathBuf) -> Result<String, Error> {
        let mut string = String::new();

        let mut file: File = match File::open(path) {
            Ok(file_handle) => file_handle,
            Err(io_error) => return Err(io_error),
        };

        match file.read_to_string(&mut string) {
            Ok(_) => (),
            Err(io_error) => return Err(io_error),
        };

        Ok(string)
    }

    if read_file_contents(PathBuf::from("src/main.rs")).is_ok() {
        println!("The program found the main file.");
    }
    if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}

fn practice_lifetime() {
    fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
        vector.push(String::from(value));
        vector.get(vector.len() - 1).unwrap()
    }

    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}

fn practice_generic() {
    struct Container<T> {
        value: T,
    }

    impl<T> Container<T> {
        pub fn new(value: T) -> Self {
            Container { value }
        }
    }

    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(
        Container::new(String::from("Bar")).value,
        String::from("Bar")
    );
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
}
