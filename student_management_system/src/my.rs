fn main() {
    let mut students = vec![
        Student {
            name: String::from("Alice"),
            age: 20,
            grade: 3.8,
        },
        Student {
            name: String::from("Bob"),
            age: 19,
            grade: 3.5,
        },
        Student {
            name: String::from("Charlie"),
            age: 21,
            grade: 3.9,
        },
        Student {
            name: String::from("David"),
            age: 20,
            grade: 3.5,
        },
        Student {
            name: String::from("Eve"),
            age: 18,
            grade: 4.0,
        },
    ];

    println!("Original student list:");
    for student in &students {
        println!("{:?}", student);
    }

    quicksort(&mut students);

    println!("\nSorted student list (by grade, age, then name):");
    for student in &students {
        println!("{:?}", student);
    }
}
