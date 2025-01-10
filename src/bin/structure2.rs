// Topic : Organizing similar data using structures

// Requirements: 
// * Print the flavor of a drink and it's fluid ounces

// Notes: 
// * Use an enum to create  different flavors of drinks
// * Use a structure to store the flavor and fluid ounces of a drink
// * Create a function that takes a drink and prints out the flavor and fluid ounces
// * Use a match statement to print the drink flavor
enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_ounces: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("Sparkling"),
        Flavor::Sweet => println!("Sweet"),
        Flavor::Fruity => println!("Fruity"),
    }
    println!("Fluid Ounces: {}", drink.fluid_ounces);
}
fn main() {
    let drink: Drink = Drink {
        flavor: Flavor::Sparkling,
        fluid_ounces: 12.0,
    };
    print_drink(drink);
    let drink: Drink = Drink {
        flavor: Flavor::Sweet,
        fluid_ounces: 16.0,
    };
    print_drink(drink);
    let drink: Drink = Drink {
        flavor: Flavor::Fruity,
        fluid_ounces: 20.0,
    };
    print_drink(drink);
}