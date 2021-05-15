use std::{
    io,
    fs::File,
    io::{
        BufRead,
        BufReader,
    },
};

pub trait ReadLines {
    fn read_lines(&self) -> io::Result<Vec<String>>;
}

impl ReadLines for File {
    fn read_lines(&self) -> io::Result<Vec<String>> {
        let mut lines_buf = BufReader::new(self).lines();
        let mut lines = Vec::<String>::new();

        while let Some(line) = lines_buf.next() {
            lines.push(line?);
        }

        Ok(lines)
    }
}