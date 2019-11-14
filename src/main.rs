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
    execute(&mut memory, &mut n);
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
                    true => result[i].p = result[i].p * BASIS + match c.to_digit(BASIS) {
                        None => 0,
                        Some(x) => x,
                    },
                    false => result[i].q = result[i].q * BASIS + match c.to_digit(BASIS) {
                        None => 0,
                        Some(x) => x,
                    },
                }
            },
            '/' => {
                p_or_q = !p_or_q;
            },
            ' ' => {
                p_or_q = !p_or_q;
                i += 1;
            },
            _ => {},
        }
    }
    return result;
}

fn execute(memory: &mut Vec<Element>, n: &mut u32) {
    let mut end_execution: bool = false;
    while memory.len() >= 1 && !end_execution {
    	let mut i: usize = 0;
			let mut none_found: bool = false;
			while i < memory.len() && !none_found {
				none_found = true;
				let p: u32 = memory[i].p;
				let q: u32 = memory[i].q;
				if *n % q == 0 {
					*n = ((*n)*p)/q;
					memory.remove(i);
					i -= 1;
					none_found = false;
				}
				i += 1;
			}
			println!("{} => {:?}", *n, *memory);
			if none_found {
				end_execution = true;
			}
		}
}
