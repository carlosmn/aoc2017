use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();

    let mut input = String::new();
    stdin.read_to_string(&mut input).expect("read from stdin");
    let initial_banks = input
        .split_whitespace()
        .map(|n| u64::from_str_radix(n, 10).expect("a number"))
        .collect::<Vec<u64>>();

    let mut cycles: Vec<Vec<u64>> = Vec::new();
    cycles.push(initial_banks);

    let mut seen_once = false;
    let mut count = 0usize;
    loop {
        let next = next_cycle(&cycles[cycles.len()-1]);
        count += 1;

        if cycles.iter().find(|&v| next == *v).is_some() {
            // If we've seen the cycle repeat once already, we exit.
            // Otherwise we reset the counter, to start counting from the
            // first repeat configuration so we know how big the loop is.
            if seen_once {
                break;
            }

            println!("Time to first repeat: {}", count);
            seen_once = true;
            count = 0;
            cycles.clear();
        }

        cycles.push(next);
    }

    println!("Cycle length {}", count);
}

/// Generate the next configuration of memory banks
fn next_cycle(pc: &[u64]) -> Vec<u64> {
    let mut c = Vec::new();
    for &v in pc {
        c.push(v);
    }

    let source = most_used(pc);
    let mut left = c[source];
    c[source] = 0;

    let mut i = source+1 as usize;
    while left > 0 {
        i = i % c.len();
        c[i] += 1;
        left -= 1;
        i += 1;
    }

    return c;
}

/// Return the position of the most used memory bank
fn most_used(c: &[u64]) -> usize {
    let mut p = 0usize;
    let mut max_v = 0u64;

    for (i, &v) in c.iter().enumerate() {
        if v > max_v {
            max_v = v;
            p = i;
        }
    }

    return p;
}
