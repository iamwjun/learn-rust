use std::fmt;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, f: fmt::Arguments) -> Result<()>;
}

fn main() {
    let x = 32;
    let y = &x;

    println!("x is {x}, y is {y}");

    *y = 12;
}

fn bar() {
    // snip
}