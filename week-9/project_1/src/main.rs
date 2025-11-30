
use std::io::Write ;

fn main() -> Result<(), Box<dyn 
std::error::Error>> {

    let header = vec![
                    "LAGER",
                    "STOUT",
                    "NON-ALCOHOLIC"]
                    ;
    let drinks = vec![
                    ["33 Export","Legend","Maltina"],
                    ["Desperados","Turbo King","Amstel Malta"],
                    ["Goldberg","Williams","Malta Gold"],
                    ["Gulder","-","Fayrouz"],
                    ["Heineken","-","-"],
                    ["Star","-","-"]
                    ];



    
   let mut file = std::fs::File::create("highest_quality_drinks.txt").expect("create failed");
   file.write_all("--------NIGERIAN BREWERY LIMITED--------".as_bytes()).expect("write failed");
   file.write_all("Here are our Highest Quality Drinks! : ".as_bytes()).expect("write failed");

   writeln!(file,)?;
     writeln!(file,"{:<15} {:<15} {:<15}",header[0][0] )?;
    

    writeln!(file, "{:<45}",".");
     writeln!(file,)?;
    for i in 0..drinks.len() { writeln!(file,"{:<15} {:<15} {:<15}",drinks[i][0],drinks[i][1],drinks[i][2]  )?;
    }

  
   println!("file written successpfully :)");


Ok(())

}
