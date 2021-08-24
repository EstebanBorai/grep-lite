use std::fmt;

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
    let term = "Abbey Road";
    let input = "\
        \"Something\" is a song by the English rock band the Beatles from
        their 1969 album Abbey Road. It was written by George Harrison,
        the band's lead guitarist. Together with his second contribution
        to Abbey Road, \"Here Comes the Sun\", it is widely viewed by music
        historians as having marked Harrison's ascendancy as a composer to
        the level of the Beatles' principal songwriters, John Lennon and
        Paul McCartney.";
    let mut lines = Lines::new(10);

    for (line_num, line) in input.lines().enumerate() {
        lines.push(Line::new(line_num, line.to_string()));

        if line.to_lowercase().contains(&term.to_lowercase()) {
            lines.print_at_index(line_num);
        }
    }
}
