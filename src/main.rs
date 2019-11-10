#[macro_use] extern crate text_io;

#[derive(Debug)]
#[derive(Clone)]
struct Element {
    p: u32,
    q: u32,
}

static BASIS: u32 = 10;

fn main() {
    print!("Welcome to fractran compiler!\n");
    print!("Enter n: ");
    let mut n: u32 = read!();
    print!("Enter the starting config: ");
    let list: String = read!("{}\n");
    let mut memory: Vec<Element> = parse_list(&list);
    execute(&memory, &n);
    print!("{:?}\n{}", memory, n);
}

fn parse_list(list: &String) -> Vec<Element> {
    let count: usize = list.matches('/').count();
    let mut result: Vec<Element> = vec![Element{p: 0, q: 0}; count];
    let mut it = list.chars().peekable();
    let mut i: usize = 0;
    let mut p_or_q: bool = true;
    while let Some(&c) = it.peek() {
        it.next();
        match c {
            '0'..='9' => {
                match p_or_q {
                    true => result[i].p = result[i].p * BASIS + c.to_digit(BASIS),
                    false => result[i].q = result[i].q * BASIS + c.to_digit(BASIS),
                }
            }
            '/' => {
                p_or_q = !p_or_q;
            }
            ' ' => {
                p_or_q = !p_or_q;
                i = i + 1;
            }
        }
    }
    return result;
}

fn execute(memory: &Vec<Element>, n: &u32) {
    println!("{:?}", memory);
}