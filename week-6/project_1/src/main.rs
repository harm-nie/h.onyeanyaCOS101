use std::io;

fn main() {
    println!("Welcome to Food by H!");
    println!("Here is our menu:");

    println!("CODE    MENU                             PRICE");
    println!("P       Pounded Yam/Edinkaiko Soup    -   ₦3200");
    println!("F       Fried Rice & Chicken          -   ₦3000");
    println!("A       Amala & Ewedu Soup            -   ₦2500");
    println!("E       Eba & Egusi Soup              -   ₦2000");
    println!("W       White Rice & Stew             -   ₦2500");

println!("\nEnter the code for the item(s) you want:");
let mut codel = String::new();
io::stdin().read_line(&mut codel).expect("Failed to read input");
let codel = codel.trim().to_uppercase();

println!("Enter the quantity you want:");
let mut quant = String::new();
io::stdin().read_line(&mut quant).expect("Failed to read quantity");
let mut  quant: f32 =  quant.trim().parse().expect("Invalid quantity entered");
println!("Quantity: {}", quant);

while quant < 1.0 {
        println!("Invalid quantity. Try again:");
        let mut new_quant = String::new();
        io::stdin().read_line(&mut new_quant).expect("Failed to read quantity");
        quant = new_quant.trim().parse().expect("Invalid number entered");
    }

    println!("Quantity: {}", quant);



let (item, price): (&str, f32) = match codel.as_str() {
        "P" => ("Pounded Yam/Edinkaiko Soup", 3200.0),
        "F" => ("Fried Rice & Chicken", 3000.0),
        "A" => ("Amala & Ewedu Soup", 2500.0),
        "E" => ("Eba & Egusi Soup", 2000.0),
        "W" => ("White Rice & Stew", 2500.0),
        _ => {
            println!("Invalid item code. Please try again.");
            return;
        }
    };


    let tea = price * quant ;
    println!("\nYour order: {}    {} X ₦{}. \nTotal cost: ₦{}.", item, quant, price, tea);


    if tea > 10000.0 {
        let dis = tea * 0.05;
        let dist = tea - dis;
        println!("\nYou get a 5% discount!");
        println!("\nDiscounted price: ₦{}",dist);
    }else {
        println!("\nTotal cost: ₦{}", tea);
    }
 



}



