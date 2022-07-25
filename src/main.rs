use std::string;

fn main() {
    println!("insert your Name: ");
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("insert your Age: ");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    let age_int: u8 = age.trim().parse().unwrap();

    // You can use the `std::io::Write` trait to write to a file.
    // get the name and age
    println!("Wellcome: {name} of {age_int} years old");
}
