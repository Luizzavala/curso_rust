fn main() {
    // i8 is for numbers positive and negative, but u8 is for only numbers positive.
    let mut edad : i8 = 36;
    let name : &str = "Luis";

    edad  = edad + 1;

    println!("Hi, my name is {name} and have {edad} years old.");
}
