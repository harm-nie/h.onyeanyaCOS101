use std::io;

fn main() {
    // ---- Store job roles using simple vectors ----
    let aps1_2 = vec!["Intern", "-", "Paralegal", "Placement"];
    let aps3_5 = vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"];
    let aps5_8 = vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"];
    let el1_8_10 = vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"];
    let el2_10_13 = vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"];
    let ses = vec!["CEO", "Dean", "Partner", "Principal"];

    let ps = vec!["Office Administrator","Academic","Lawyer","Teacher"];
    println!( "{:?}", ps);
    let mut job = String::new();
    println!("Enter job: ");
    io::stdin().read_line(&mut job).expect("Failed to read input");
    let job = job.trim().to_lowercase();

    for i in 0..4 {
        if job == ps[i].to_lowercase() {
        println!("{}, {}, {}, {}, {}, {}", aps1_2[i], aps3_5[i], aps5_8[i], el1_8_10[i], el2_10_13[i], ses[i]);
        }
    }

    // ---- Get role from user ----
    let mut input = String::new();
    println!("Enter staff role:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_lowercase();
    let role:&str = input.as_str();

    // ---- Get years of experience ----
    let mut years = String::new();
    println!("Enter years of experience:");
    io::stdin().read_line(&mut years).expect("Failed to read input");
    let years: u32 = years.trim().parse().expect("Failed to read input");

    // ---- Check level (simple if statements) ----
    if aps1_2.contains(&role) && years <= 2 {
        println!("APS Level: APS 1-2");
    }
    else if aps3_5.contains(&role) && years >= 3 && years <= 5 {
        println!("APS Level: APS 3-5");
    }
    else if aps5_8.contains(&role) && years >= 5 && years <= 8 {
        println!("APS Level: APS 5-8");
    }
    else if el1_8_10.contains(&role) && years >= 8 && years <= 10 {
        println!("APS Level: EL1 8-10");
    }
    else if el2_10_13.contains(&role) && years >= 10 && years <= 13 {
        println!("APS Level: EL2 10-13");
    }
    else if ses.contains(&role) && years >= 14 {
        println!("APS Level: SES");
    }
    else {
        println!("No matching APS level found.");
    }
}

