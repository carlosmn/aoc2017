use std::ops::Range;

fn main() {
    let stdin = std::io::stdin();
    let len = usize::from_str_radix(&std::env::args().nth(1).expect("arg1"), 10).expect("len");

    let mut raw_input = String::new();
    stdin.read_line(&mut raw_input).expect("input line");
    let line = raw_input.trim();

    let lengths = line
        .split(',')
        .map(|s| { println!("s {:?}", s); s})
        .map(|s| usize::from_str_radix(s, 10).expect("number"))
        .collect::<Vec<usize>>();

    let mut list: Vec<usize> = (0..len).collect();
    let mut skip = 0usize;
    let mut pos = 0usize;

    println!("before: {:?}", list);

    for len in lengths {
        println!("pos: {}, skip: {}, len: {}", pos, skip, len);
        reverse_range(&mut list, pos, len);
        pos = (pos + len + skip) % list.len();

        println!("intermediate: {:?}", list);
        skip += 1;
    }

    println!("checksum {}", list[0] * list[1]);
}

/// Reverse a particular range in the vector
fn reverse_range(v: &mut Vec<usize>, pos: usize, len: usize) {
    let mut start = pos;
    let mut end = ((pos + len) % v.len()) - 1;

    for _ in (0..len/2) {
        println!("start: {}, end: {}", start, end);
        println!("revr: {:?}", v);
        let (a, b) = (v[start], v[end]);

        v[start] = b;
        v[end] = a;

        start += 1;
        if start >= v.len() {
            start = 0;
        }
        end = end.checked_sub(1).unwrap_or(v.len()-1);
    }
}
