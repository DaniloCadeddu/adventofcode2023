use std::fs;

pub fn get_input(input_file_name: &str) -> Vec<String> {
    let mut input_path = String::from("./input/");
    input_path.push_str(input_file_name);

    fs::read_to_string(input_path)
        .expect("Cannot read input file")
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}