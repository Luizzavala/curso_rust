use std::{io::Stdout, string};

fn main() {
    println!("insert your Age: ");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    // parse the age into a integer
    let age_int : u8 = age.trim().parse().unwrap();

    if age_int >= 18 {
        println!("your age is {}, enjoi your membership", age_int);
    } else {
        println!("your age is {}, you cant pass", age_int);
    }
}
