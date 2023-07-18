use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = match args.get(1) {
        Some(path) => path,
        None => panic!("Expected file path after 'format'"),
    };
    
    let input = fs::read_to_string(path)
        .unwrap_or_else(|path| panic!("Unable to read file: {}", path));
    
    let output = format_string(input);

    fs::write(path, output)
        .unwrap_or_else(|path| panic!("Unable to write to file: {}", path));
}

fn format_string(input: String) -> String {
    let mut formatted_string = String::new();
    
    for mut line in input.lines() {
        if line.is_empty() {continue}
        line = line.trim_start_matches(|c: char| !c.is_alphabetic());
        line = line.trim_end_matches(|c: char| !c.is_alphabetic());
        let formatted_line = &format!("### {}\n\n\n\n", &line);
        formatted_string.push_str(formatted_line)
    }

    formatted_string
}
