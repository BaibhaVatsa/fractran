#[macro_use] extern crate text_io;

#[derive(Debug)]
struct Element {
    p: i128,
    q: i128,
}

fn main() {
    print!("Welcome to fractran compiler!\nEnter n: ");
    let mut n: i128 = read!();
    print!("Enter the starting config: ");
    let list: String = read!("{}\n");
    let mut memory: Vec<Element> = parse_list(list);
    let result = execute(memory, n);
    memory = result.0;
    n = result.1;
    print!("{:?}\n{}", memory, n);
}

fn parse_list(list: String) -> Vec<Element> {
    list = list[1...];
    return vec![Element{p: 5, q: 6}, Element{p: 5, q: 6}, Element{p: 5, q: 6}];
}

fn execute(memory: Vec<Element>, n: i128) -> (Vec<Element>, i128) {
    return (vec![Element{p: 5, q: 6}, Element{p: 5, q: 6}, Element{p: 5, q: 6}], 0);
}