use std::io::Read;
use std::io::Write;

fn main() {


    let categories = "   LAGER           STOUT            NON-ALCOHOLIC\n ";

    let drink = "   33 Export         Legend            Maltina\n
    Desparados        Turbo King             Amstel Malta\n
    Goldberg          Williams          Malta Gold\n
    Gulder            -                  Fayrouz\n
    Heineken          -                 -\n
    Star              -                 -\n";

let mut file = std::fs::File::create("stuff.txt").expect("create failed");
    file.write_all("<-------NIGERIAN BREWRIES LIMITED------->\n".as_bytes()).expect("wrtie failed");
    file.write_all(categories.as_bytes()).expect("write failed");
    file.write_all(drink.as_bytes()).expect("write failed");
    println!("\nData written to file.");

let mut file = std::fs::File::open("stuff.txt").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
println!("{}", contents)

}