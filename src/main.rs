use std::env::args;
use std::fmt;
use std::process::exit;

struct Line {
    index: usize,
    value: String,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.index + 1, self.value)
    }
}

struct Lines(Vec<Line>);

impl Lines {
    pub fn new(capacity: usize) -> Self {
        Lines(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, value: Line) {
        self.0.push(value)
    }

    pub fn print_at_index(&self, index: usize) {
        if index > self.0.len() - 1 {
            // A better error handling involves using `Result` where
            // we could return a `Result::Err` variant to the consumer
            //
            // For simplicity sake, we use `panic!` instead which will
            // break the application
            panic!("Out of bounds");
        }

        let previous = index.saturating_sub(1);

        if previous == 0 && index != 0 {
            println!("{}", self.0.get(previous).unwrap());
        }

        println!("{}", self.0.get(index).unwrap());

        if index + 1 <= self.0.len() - 1 {
            println!("{}", self.0.get(index + 1).unwrap());
        }
    }
}

impl Line {
    pub fn new(index: usize, value: String) -> Self {
        Line { index, value }
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        eprintln!("Expected 2 arguments. `term` and `input` are required arguments.");
        exit(1);
    }

    let term = &args[1];
    let input = &args[2];

    let mut lines = Lines::new(10);

    for (line_num, line) in input.lines().enumerate() {
        lines.push(Line::new(line_num, line.to_string()));

        if line.to_lowercase().contains(&term.to_lowercase()) {
            lines.print_at_index(line_num);
        }
    }
}
