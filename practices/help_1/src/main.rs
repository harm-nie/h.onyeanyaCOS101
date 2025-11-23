use std::io;

fn get_input(prompt: &str) -> i32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let value = input.trim().parse().expect("Invalid input. Please enter a number.");
    return value;
}

fn main (){
    println!("<------Public Service APS level Checker------>");

    let ps = vec!["Office Administrator","Academic","Lawyer","Teacher"];
    println!( "{:?}", ps);

    let mut input1 = String::new();
    println!("Enter your job:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    input1 = input1.trim().to_lowercase();

    let lvl = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];


    let (admin,academic,lawyer,teacher): (&str,&str,&str,&str) = match input1.to_lowercase().as_str() {
        "APS 1-2" => ("Intern","-","Paralegal","Placement"),
        "APS 3-5" => ("Administrator","Research Assistant","Junior Associate","Classroom Teacher"),
        "APS 5-8" => ("Senior Administrator","PhD Candidate","Associate","Snr Teacher"),
        "EL1 8-10" => ("Office Manager","Post-Doc Researcher","Senior Associate 1-2","Leading Teacher" ),
        "EL2 10-13" => ("Director","Senior Lecturer","Senior Associate 3-4","Deputy Principal"),
        "SES" => ("CEO","Dean","Partner","Principal"),
        _ => {
            println!("Invalid lvl. Please try again.");
            return;
        }
    };

    let admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let academic = vec!["-","Research Assistant","Phd Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    let teacher = vec!["Placement","Classroom Teacher","Snr Teacher","Leading Teacher","Leading Teacher","Deputy Principal","Principal"];

}
