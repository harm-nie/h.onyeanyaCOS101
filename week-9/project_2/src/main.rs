use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() -> io::Result<()> {
    // Sample student data
    let students = vec![
        Student {
            name: "Oluchi Nnordi".to_string(),
            matric_number: "ACC1021111".to_string(),
            department: "Accounting".to_string(),
            level: 200,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO2010001".to_string(),
            department: "Economics".to_string(),
            level: 200,
        },
        Student {
            name: "Shams Abiodun".to_string(),
            matric_number: "ECO2010002".to_string(),
            department: "Economics".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gode".to_string(),
            matric_number: "ELE2020001".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric_number: "MEE1020001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    // Display student details
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric Number", "Department", "Level");
    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        );
    }

    // Save to file
    let mut file = File::create("pau_smis.txt")?;
    writeln!(file, "{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric Number", "Department", "Level")?;
    for student in students {
        writeln!(
            file,
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        )?;
    }

    println!("\nStudent data saved to 'pau_smis.txt'");
    Ok(())
}

