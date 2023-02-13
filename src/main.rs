use std::{
    fs::OpenOptions,
    io::{self, BufRead, BufReader},
};

fn read_from_pipe() -> Vec<String> {
    let mut lines: Vec<String> = vec![];

    for (i, line) in io::stdin().lock().lines().enumerate() {
        let l = line.expect("extected input");
        println!("[{i}] {}", l);
        lines.push(l);
    }

    lines
}

fn read_input() -> io::Result<String> {
    let file = OpenOptions::new().read(true).open("/dev/tty")?;
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    buf_reader.read_line(&mut line)?;
    Ok(line)
}

fn main() {
    let lines = read_from_pipe();

    loop {
        let input = read_input().expect("fail to read user input");

        let i: usize = match input.trim().parse() {
            Ok(n) => n,
            _ => {
                println!("fail to parse");
                continue;
            }
        };

        match lines.get(i) {
            Some(line) => {
                println!("{}", line)
            }
            None => {
                println!("please provide a valid range");
                continue;
            }
        }
    }
}
