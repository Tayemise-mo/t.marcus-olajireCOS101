use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Student_Management_Information_System {
    Student_Name: String,
    Matric_Number: String,
    Department: String,
    Level: u32,
}
fn main() {
    //Creating Sample Data
    let PAU_SMIS = vec![
        Student_Management_Information_System {
            Student_Name: "Oluch Mordi".to_string(),
            Matric_Number: "ACC10211111".to_string(),
            Department: "Accounting".to_string(),
             Level: 300,
        },
        Student_Management_Information_System {
            Student_Name: "Adams Aliyu".to_string(),
            Matric_Number: "ECO10110101".to_string(),
            Department: "Economics".to_string(),
             Level: 100,
        },
        Student_Management_Information_System {
            Student_Name: "Shania Bolade".to_string(),
            Matric_Number: "CSC10328828".to_string(),
            Department: "Computer".to_string(),
             Level: 200,
        },
        Student_Management_Information_System {
            Student_Name: "Adekunle Gold".to_string(),
            Matric_Number: "EEE11020202".to_string(),
            Department: "Electrical".to_string(),
             Level: 200,
        },
        Student_Management_Information_System {
            Student_Name: "Blanca Edemoh".to_string(),
            Matric_Number: "MEE10202001".to_string(),
            Department: "Mechanical".to_string(),
            Level: 100,
        },
    ];

    let table = Table::new(PAU_SMIS);

    println!("{}", table);
}
