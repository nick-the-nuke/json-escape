use std::io::{self, Read};

fn main() {
    let mut no_escape_slash = false;
    for arg in std::env::args().skip(1) {
        if arg == "--no-escape-slash" {
            no_escape_slash = true;
        }
    }
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");
    // Strip trailing newline added by shell piping
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    let escaped: String = json_escape::escape_str(&input).collect();
    let result = if no_escape_slash {
        escaped
    } else {
        escaped.replace('/', "\\/")
    };
    println!("\"{}\"", result);
}
