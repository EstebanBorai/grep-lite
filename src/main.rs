struct Line {
    index: usize,
    value: String,
}

impl Line {
    pub fn new(index: usize, value: String) -> Self {
        Line { index, value }
    }
}

fn main() {
    let term = "something";
    let input = "\
        \"Something\" is a song by the English rock band the Beatles from
        their 1969 album Abbey Road. It was written by George Harrison,
        the band's lead guitarist. Together with his second contribution
        to Abbey Road, \"Here Comes the Sun\", it is widely viewed by music
        historians as having marked Harrison's ascendancy as a composer to
        the level of the Beatles' principal songwriters, John Lennon and
        Paul McCartney.";
    let mut lines: Vec<Line> = Vec::with_capacity(10);

    for (line_num, line) in input.lines().enumerate() {
        lines.push(Line::new(line_num, line.to_string()));

        if line.to_lowercase().contains(term) {
            println!("{}: {}", line_num, line);
        }
    }
}
