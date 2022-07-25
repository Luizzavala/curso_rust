use std::{io::Stdout, string};
fn main() {
    let number_1 = 123;  
    let number_2 = 123;

    let sum = number_1 + number_2;
    
    loop {
        //mostrar los dos numeros en consola
        println!("First number: {}, Second number: {},", number_1, number_2);
        
        //Obtener del usuario el numero que represnta la suma
        let mut user_sum = String::new();
        std::io::stdin().read_line(&mut user_sum).unwrap();
        let user_sum_int: i32 = user_sum.trim().parse().unwrap();
        
        
        if user_sum_int == sum {
            println!("that sum its correct:");
            break;
        } else {
            println!("This sum its incorrect: that sum is{}", sum);
        }
    }
}