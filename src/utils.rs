use std::fs::File;
use std::io::Read;

pub fn read_file(path: String) -> String {
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Fail to load file at {path}"));
    let mut data = String::new();
    file.read_to_string(&mut data)
        .unwrap_or_else(|_| panic!("Failed to read file at {path}"));
    data
}

pub fn read_from_stdin() -> String {
    println!("Please paste your input here:");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input
}

pub fn transpose<T: Clone>(records: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = vec![Vec::new(); records.iter().map(|record| record.len()).max().unwrap()];

    for record in records {
        for (index, element) in record.iter().enumerate() {
            transposed[index].push(element.clone());
        }
    }
    transposed
}
