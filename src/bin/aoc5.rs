use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();

    let mut input = String::new();
    stdin.read_to_string(&mut input).expect("read from stdin");
    let mut jumps = input.split_whitespace().map(|n| isize::from_str_radix(n, 10).expect("a number")).collect::<Vec<isize>>();

    let mut offset = 0usize;
    let mut steps = 0u64;
    loop {
        steps += 1;
        let next_offset: isize = offset as isize + jumps[offset];

        // Part 1:
        // jumps[offset] += 1;

        //Now part 2:
        if jumps[offset] >= 3 {
            jumps[offset] -= 1;
        } else {
            jumps[offset] += 1;
        }

        if next_offset < 0 || next_offset >= jumps.len() as isize {
            break;
        }

        offset = next_offset as usize;
    }

    println!("steps {}", steps);
}
