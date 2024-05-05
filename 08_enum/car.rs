#[derive(Debug)]
enum CarType {
    Hatch,
    Sedan,
}

fn print_s(car: CarType) {
    match car {
        CarType::Hatch => {
            println!("Small");
        },
        CarType::Sedan => {
            println!("Medium");
        }
    }
}

fn main() {
    let car1 = CarType::Hatch;
    let car2 = CarType::Sedan;
    
    println!("Car 1:");
    print_s(car1);
    
    println!("Car 2:");
    print_s(car2);
}

