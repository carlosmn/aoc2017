use std::fmt::Write;

fn main() {
    let stdin = std::io::stdin();
    let mut args = std::env::args();
    let ascii = if let Some(ref v) = args.nth(1) {
        v == "--ascii"
    } else {
        false
    };

    let len = if ascii {
        usize::from_str_radix(&std::env::args().nth(2).expect("arg1"), 10).expect("len")
    } else {
        usize::from_str_radix(&std::env::args().nth(1).expect("arg1"), 10).expect("len")
    };


    let mut raw_input = String::new();
    stdin.read_line(&mut raw_input).expect("input line");
    let line = raw_input.trim();

    let lengths = if ascii {
        let mut l = line
            .bytes()
            .map(|b| b as usize)
            .collect::<Vec<usize>>();
        l.append(&mut vec![17, 31, 73, 47, 23]);
        l
    } else {
        line
            .split(',')
            .map(|s| usize::from_str_radix(s, 10).expect("number"))
            .collect::<Vec<usize>>()
    };

    let mut list: Vec<usize> = (0..len).collect();
    let mut skip = 0usize;
    let mut pos = 0usize;

    for _ in 0..64 {
        for len in &lengths {
            reverse_range(&mut list, pos, len.clone());
            pos = (pos + len + skip) % list.len();

            skip += 1;
        }
    }

    println!("checksum {}", list[0] * list[1]);

    let as_u8s = to_u8(&list);
    let dense = dense_hash(&as_u8s);
    let hex = to_hex(&dense);

    println!("sparse: {:?}", as_u8s);
    println!("dense: {}", hex);
}

/// Convert a vector of usize into u8, panicking if the values are too large.
fn to_u8(v: &Vec<usize>) -> Vec<u8> {
    v.iter().map(|b| {
        if *b > 255 {
            panic!("value is too large");
        }

        *b as u8
    }).collect()
}

/// Creates a "dense hash" of 16 numbers out of the sparse hash given as input.
fn dense_hash(v: &Vec<u8>) -> Vec<u8> {
    let mut dense = Vec::new();
    for chunk in v.chunks(16) {
        let h = chunk.iter().fold(0, |acc, &x| { println!("{} ^ {} = {}", acc, x, acc ^ x); acc ^ x});
        println!("chunk {:?}, h {:?}", chunk, h);
        dense.push(h);
    }

    dense
}

/// Creates a hexadecimal string out of the numbers in the vector
fn to_hex(v: &Vec<u8>) -> String {
    let mut hex = String::new();
    for n in v {
        write!(&mut hex, "{:02x}", *n).expect("writing hex");
    }

    hex
}

/// Reverse a particular range in the vector
fn reverse_range(v: &mut Vec<usize>, pos: usize, len: usize) {
    let mut start = pos;
    let mut end = ((pos + len) % v.len()).checked_sub(1).unwrap_or(v.len()-1);

    for _ in 0..len/2 {
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
