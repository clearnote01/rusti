#[derive(Debug)]
enum List {
    Empty,
    Elem(i32, Box<List>),
}

fn main() {
    println!("Hello, world!");
}
