fn main() {
    let developers = vec![ ("Harmonie", 3),("James", 5),("Fatima", 2),("Kika", 8),("Kevin", 6),
    ];

    let mut highest_name = "";
    let mut highest_years = 0;

    // Loop through the vector
    for dev in developers {
        let (name, years) = dev;

        if years > highest_years {
            highest_years = years;
            highest_name = name;
        }
    }

    println!("Most experienced developer is {} with {} years.", highest_name, highest_years);
}

