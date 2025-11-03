use std::io;

fn main() {
    let mut level =String::new();
    let mut name =String::new();
    let mut age =String::new();

    println!("enter your name please:");
    io::stdin().read_line(&mut name).expect("this is not a valid input");

    println!("are you 18? (yes/no):");
    io::stdin().read_line(&mut age).expect("this is not a valid input");
    let real_age= age.trim().to_lowercase();

    println!("level:");
    io::stdin().read_line(&mut level).expect("this is not a valid input");
    let real_level:i32 = level.trim().parse().expect("not valid");

    if real_level ==100 && real_age >= 18.to_string() {
        println!("{}you are welcome to the party", name );
    } else {
        println!("you are underage sorry");
    }


}
