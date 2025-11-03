fn main() {
    let name1 = "Harmonie Adesokan";
    println!("My name is {}", name1);

    //find and replace
    let name2 = name1.replace("Harmonie", "Chim-maranma");
    println!("You can also call me {}", name2);
    let faculty = "Faculty of Science and Technology";

    //find and replace
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}", school);
}
