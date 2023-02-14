use std::{
    fs::OpenOptions,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

fn read_from_pipe() -> Vec<String> {
    let mut lines: Vec<String> = vec![];

    for (i, line) in io::stdin().lock().lines().enumerate() {
        let l = line.expect("extected input");

        let file = OpenOptions::new()
            .write(true)
            .open("/dev/tty")
            .expect("fail to open file");

        let mut buf_writer = BufWriter::new(file);
        // TODO: format this crap
        let _ = buf_writer.write(format!("[{i}] {l}\n").as_bytes());
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
                continue;
            }
        };

        match lines.get(i) {
            Some(line) => {
                let _ = writeln!(io::stdout(), "{}", line);
                break;
            }
            None => continue,
        }
    }
}
