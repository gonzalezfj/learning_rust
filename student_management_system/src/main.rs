#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    grade: f32,
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        // ieee 754 floating point comparison
        (self.grade - other.grade).abs() < f32::EPSILON
            && self.age == other.age
            && self.name == other.name
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Sort by grade first, then by age, then by name
        self.grade.partial_cmp(&other.grade).map(|o| {
            o.then(self.age.cmp(&other.age))
                .then(self.name.cmp(&other.name))
        })
    }
}

impl Eq for Student {}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}

fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_idx = partition(arr);

    let (left, right) = arr.split_at_mut(pivot_idx);
    quicksort(left);
    quicksort(&mut right[1..]);
}

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
