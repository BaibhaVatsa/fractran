#[macro_use]
extern crate text_io;

#[derive(Debug, Clone)]
struct Element {
    p: u32,
    q: u32,
}

static BASIS: u32 = 10;

fn main() {
    println!("Welcome to fractran compiler!");
    println!("Enter n");
    let mut n: u32 = read!();
    println!("Enter the starting config");
    let list: String = read!("{}\n");
    let mut memory: Vec<Element> = parse_list(&list);
    execute(&mut memory, &mut n);
    println!("{}", n);
}

fn parse_list(list: &str) -> Vec<Element> {
    let count: usize = list.matches('/').count();
    let mut result: Vec<Element> = vec![Element { p: 0, q: 0 }; count];
    let mut it = list.chars().peekable();
    let mut i: usize = 0;
    let mut p_or_q: bool = true;
    while let Some(&c) = it.peek() {
        it.next();
        match c {
            '0'..='9' => {
                if p_or_q {
                    result[i].p = result[i].p * BASIS
                        + match c.to_digit(BASIS) {
                            None => 0,
                            Some(x) => x,
                        }
                } else {
                    result[i].q = result[i].q * BASIS
                        + match c.to_digit(BASIS) {
                            None => 0,
                            Some(x) => x,
                        }
                }
            }
            '/' => {
                p_or_q = !p_or_q;
            }
            ' ' => {
                if result[i].p % result[i].q == 0 {
                    result[i].p /= result[i].q;
                    result[i].q = 1;
                }
                p_or_q = !p_or_q;
                i += 1;
            }
            _ => {}
        }
    }
    result
}

fn execute(memory: &mut Vec<Element>, n: &mut u32) {
    let mut n_val: u32 = *n;
    let mut i: usize = 0;
    while i < memory.len() {
        let p: u32 = memory[i].p;
        let q: u32 = memory[i].q;
        if n_val % q == 0 {
            n_val = (n_val * p) / q;
        } else {
            i += 1;
        }
    }
    *n = n_val;
}
